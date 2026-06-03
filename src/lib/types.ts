import type { validInput } from "./validation";

export const FieldKind = ["text", "number", "date"] as const;

export type Field = {
	name: string;
	kind: (typeof FieldKind)[number];
};

export type AssetField = {
	name: string;
	value: ReturnType<typeof validInput>;
};

export type Model = {
	uuid: string;
	name: string;
	fields: Field[];
};

export type Asset = {
	uuid: string;
	name: string;
	fields: AssetField[];
};

export type AssetWithModel = {
	uuid: string;
	name: string;
	fields: AssetField[];
	model_uuid: string;
	model_name: string;
	model_fields: Field[];
};
