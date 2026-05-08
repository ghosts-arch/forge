import type { PgliteDatabase } from "drizzle-orm/pglite";
import * as schema from "$lib/server/database/schema";
import { eq } from "drizzle-orm";

export class ModelRepository {
	constructor(private readonly database: PgliteDatabase<typeof schema>) {
		this.database = database;
	}

	getModels = async () => {
		try {
			const models = await this.database.select().from(schema.modelsTable);
			return models;
		} catch (err) {
			console.error(err);
		}
	};

	deleteModel = async (uuid: string) => {
		try {
			await this.database
				.delete(schema.modelsTable)
				.where(eq(schema.modelsTable.id, uuid));
		} catch (err) {
			console.error(err);
		}
	};
}
