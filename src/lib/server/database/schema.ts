import type { AssetField, Field } from "$lib/types";
import { index, jsonb, pgTable, uuid, varchar } from "drizzle-orm/pg-core";

export const assetsTable = pgTable(
	"assets",
	{
		id: uuid().primaryKey().defaultRandom(),
		modelId: uuid().references(() => modelsTable.id, { onDelete: "set null" }),
		name: varchar({ length: 255 }).notNull(),
		fields: jsonb()
			.$type<AssetField[]>()
			.notNull()
			.$defaultFn(() => []),
	},
	(t) => ({
		researchIndex: index("research_index").using("gin", t.fields),
	}),
);

export const modelsTable = pgTable("models", {
	id: uuid().primaryKey().defaultRandom(),
	name: varchar({ length: 255 }).notNull(),
	fields: jsonb()
		.$type<Field[]>()
		.notNull()
		.$defaultFn(() => []),
});
