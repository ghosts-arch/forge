export const FieldKind = ["text", "number", "date"] as const;

export type Field = {
	name: string;
	type: (typeof FieldKind)[number];
};

export type AssetField = {
	name: string;
	// biome-ignore lint/suspicious/noExplicitAny: fix later
	value: any;
};

export type Model = {
	uuid: string;
	name: string;
	fields: Field[];
};

export type Asset = {
	id: string;
	name: string;
	fields: AssetField[];
};
