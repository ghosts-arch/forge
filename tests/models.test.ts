import { ModelRepository } from "$lib/server/infrastructure/model/model.repository";
import { PGlite } from "@electric-sql/pglite";
import * as schema from "$lib/server/database/schema";
import { relations } from "$lib/server/database/relations";
import { test, beforeEach, describe, expect } from "bun:test";
import { drizzle, PgliteDatabase } from "drizzle-orm/pglite";
import { migrate } from "drizzle-orm/pglite/migrator";
import { eq } from "drizzle-orm";

let modelRepository: ModelRepository;
let database: PgliteDatabase<typeof schema>;
let client: PGlite;

describe("testing modelRepository", () => {
	beforeEach(async () => {
		client = new PGlite();
		database = drizzle({ client, schema, relations });
		await migrate(database, { migrationsFolder: "drizzle" });
		modelRepository = new ModelRepository(database);
	});

	test("delete model", async () => {
		const [deletedModel] = await database
			.insert(schema.modelsTable)
			.values({ name: "to_delete" })
			.returning({ insertedId: schema.modelsTable.id });
		await modelRepository.deleteModel(deletedModel.insertedId);
		const [isDeletedModel] = await database
			.select()
			.from(schema.modelsTable)
			.where(eq(schema.modelsTable.id, deletedModel.insertedId));
		expect(isDeletedModel).toBeUndefined();
	});
});
