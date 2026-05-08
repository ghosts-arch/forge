import { command, form, query, requested } from "$app/server";
import { ModelService } from "$lib/core/model/model.service";
import { db } from "$lib/server/database/database";
import { getModel } from "$lib/server/database/functions";
import { modelsTable } from "$lib/server/database/schema";
import { ModelRepository } from "$lib/server/infrastructure/model/model.repository";
import { FieldKind } from "$lib/types";
import { redirect } from "@sveltejs/kit";
import { eq } from "drizzle-orm";
import * as v from "valibot";

const modelRepository = new ModelRepository(db);
const modelService = new ModelService(modelRepository);

export const createModel = form(
	v.object({ name: v.string() }),
	async (name) => {
		try {
			await db.insert(modelsTable).values(name);
		} catch (err) {
			console.error(err);
		}
		redirect(303, "/");
	},
);

export const getModels = query(async () => {
	const models = await modelService.getModels();
	return models;
});

export const deleteModel = command(v.string(), async (uuid) => {
	await modelService.deleteModel(uuid);
	await requested(getModels, 1).refreshAll();
});

export const getModelQuery = query(v.string(), async (uuid) => {
	const model = await getModel(uuid);
	return model;
});

export const addField = form(
	v.object({
		uuid: v.string(),
		name: v.string(),
		type: v.picklist(FieldKind),
	}),
	async ({ uuid, name, type }) => {
		const model = await getModel(uuid);
		if (!model.fields) model.fields = [];
		const updatedFields = [...model.fields, { name, type }];
		await db
			.update(modelsTable)
			.set({ fields: updatedFields })
			.where(eq(modelsTable.id, uuid));
	},
);
