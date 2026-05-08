import { eq } from "drizzle-orm";
import { db } from "./database";
import { assetsTable, modelsTable } from "./schema";

export const getModel = async (uuid: string) => {
	const [model] = await db
		.select()
		.from(modelsTable)
		.where(eq(modelsTable.id, uuid));
	return model;
};

export const getAsset = async (uuid: string) => {
	const [asset] = await db
		.select()
		.from(assetsTable)
		.where(eq(assetsTable.id, uuid));
	if (!asset) throw Error();
	return asset;
};

export const getAssetWithModel = async (uuid: string) => {
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
};
