import { type validInput } from "./utils/validation";

export const FieldKind = ["text", "number", "date"] as const;

export type Field = {
	name: string;
	type: (typeof FieldKind)[number];
};

export type AssetField = {
	name: string;
	value: ReturnType<typeof validInput>;
};
