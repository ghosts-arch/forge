import { FieldKind } from "$lib/types";
import * as v from "valibot";

export const NumberSchema = v.pipe(v.string(), v.transform(Number), v.number());
export const TextSchema = v.string();
export const DateSchema = v.pipe(
	v.string(),
	v.transform((input) => new Date(input)),
	v.date(),
);

export const parseValidationSchema = {
	text: TextSchema,
	number: NumberSchema,
	date: DateSchema,
};
const isValidFieldKind = (
	expectedType: string,
): expectedType is (typeof FieldKind)[number] => {
	return v.is(v.picklist(FieldKind), expectedType);
};

export const validInput = (expectedType: string, receivedValue: unknown) => {
	if (!isValidFieldKind(expectedType)) throw Error();
	return v.parse(parseValidationSchema[expectedType], receivedValue);
};
