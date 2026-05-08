import { defineRelations } from "drizzle-orm";
import * as schema from "./schema";

export const relations = defineRelations(schema, (r) => ({
	assetsTable: {
		model: r.one.modelsTable({
			from: r.assetsTable.modelId,
			to: r.modelsTable.id,
		}),
	},
	modelsTable: {
		assets: r.many.assetsTable(),
	},
}));
