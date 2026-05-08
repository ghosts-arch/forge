import { command, form, query } from "$app/server";

import { db } from "$lib/server/database/database";
import { getAssetWithModel, getModel } from "$lib/server/database/functions";
import { assetsTable } from "$lib/server/database/schema";

import { type AssetField, type Field } from "$lib/types";
import { validInput } from "$lib/utils/validation";
import { arrayContains, eq } from "drizzle-orm";

import * as v from "valibot";

export const createAsset = form(
	v.object({
		name: v.string(),
		modelId: v.string(),
		fields: v.array(v.object({ name: v.string(), value: v.string() })),
	}),
	async (data) => {
		try {
			const { name, modelId, fields } = data;
			const model = await getModel(modelId);
			const createdFields: AssetField[] = [];
			let currentField: Field | undefined;
			let parsedValue: ReturnType<typeof validInput>;
			for (const field of fields) {
				currentField = model.fields.find(
					(modelField) => modelField.name === field.name,
				);
				if (!currentField) throw new Error();
				parsedValue = validInput(currentField.type, field.value);
				createdFields.push({
					name: field.name,
					value: parsedValue,
				});
			}
			await db
				.insert(assetsTable)
				.values({ name, modelId, fields: createdFields });
		} catch (err) {
			console.log(err);
		}
	},
);

export const searchAssets = command(v.string(), async (input) => {
	if (!input) return [];
	const assets = await db
		.select()
		.from(assetsTable)
		.where(
			arrayContains(assetsTable.fields, [{ value: input }] as AssetField[]),
		);
	console.log(assets);
	return assets;
});

export const getAssetQuery = query(v.string(), async (uuid) => {
	const asset = await db.query.assetsTable.findFirst({
		with: {
			model: true,
		},
		where: {
			id: { eq: uuid },
		},
	});
	if (!asset) throw Error();
	return asset;
});

export const addAssetField = form(
	v.object({ uuid: v.string(), name: v.string(), value: v.string() }),
	async ({ uuid, name, value }) => {
		const asset = await getAssetWithModel(uuid);
		const fieldExpectedKind = asset.model?.fields.filter(
			(field) => field.name === name,
		)[0];
		if (!fieldExpectedKind) throw new Error();
		const parsedValue = validInput(fieldExpectedKind.type, value);
		const currentFields = asset.fields || [];
		const updatedFields = currentFields.map((field) =>
			field.name === name ? { name, value: parsedValue } : field,
		);
		const fieldExists = currentFields.some((field) => field.name === name);
		if (!fieldExists) updatedFields.push({ name, value: parsedValue });
		await db
			.update(assetsTable)
			.set({ fields: updatedFields })
			.where(eq(assetsTable.id, uuid));
	},
);

export const getAssetsQuery = query(async () => {
	const assets = await db.query.assetsTable.findMany({
		with: {
			model: true,
		},
	});
	if (!assets) throw Error();
	return assets;
});
