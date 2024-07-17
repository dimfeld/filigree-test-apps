import { client, type ModelDefinition } from "filigree-svelte";
import { z } from "zod";
import { ObjectPermission } from "../model_types.js";

export const ReportTagSchema = z.object({
	report_id: z.string(),
	tag_id: z.string(),
	organization_id: z.string(),
	updated_at: z.string().datetime(),
	created_at: z.string().datetime(),
});

export type ReportTag = z.infer<typeof ReportTagSchema>;
export const ReportTagListResultSchema = ReportTagSchema;
export type ReportTagListResult = ReportTag;
export const ReportTagPopulatedGetResultSchema = ReportTagSchema;
export type ReportTagPopulatedGetResult = ReportTag;
export const ReportTagPopulatedListResultSchema = ReportTagSchema;
export type ReportTagPopulatedListResult = ReportTag;
export const ReportTagCreateResultSchema = ReportTagSchema;
export type ReportTagCreateResult = ReportTag;

export const ReportTagCreatePayloadAndUpdatePayloadSchema = z.object({
	report_id: z.string().optional(),
	tag_id: z.string().optional(),
});

export type ReportTagCreatePayloadAndUpdatePayload = z.infer<
	typeof ReportTagCreatePayloadAndUpdatePayloadSchema
>;
export const ReportTagCreatePayloadSchema =
	ReportTagCreatePayloadAndUpdatePayloadSchema;
export type ReportTagCreatePayload = ReportTagCreatePayloadAndUpdatePayload;
export const ReportTagUpdatePayloadSchema =
	ReportTagCreatePayloadAndUpdatePayloadSchema;
export type ReportTagUpdatePayload = ReportTagCreatePayloadAndUpdatePayload;

export const baseUrl = "report_tags";
export const urlWithId = (id: string) => `${baseUrl}/${id}`;

export const urls = {
	create: baseUrl,
	list: baseUrl,
	get: urlWithId,
	update: urlWithId,
	delete: urlWithId,
};

export const ReportTagModel: ModelDefinition<typeof ReportTagSchema> = {
	name: "ReportTag",
	plural: "ReportTags",
	baseUrl,
	urls,
	schema: ReportTagSchema,
	createSchema: ReportTagCreatePayloadSchema,
	updateSchema: ReportTagUpdatePayloadSchema,
	fields: [
		{
			name: "report_id",
			type: "uuid",
			label: "Report Id",
			constraints: {
				required: true,
			},
		},
		{
			name: "tag_id",
			type: "uuid",
			label: "Tag Id",
			constraints: {
				required: true,
			},
		},
		{
			name: "organization_id",
			type: "uuid",
			label: "Organization Id",
			constraints: {
				required: true,
			},
		},
		{
			name: "updated_at",
			type: "date-time",
			label: "Updated At",
			constraints: {
				required: true,
			},
		},
		{
			name: "created_at",
			type: "date-time",
			label: "Created At",
			constraints: {
				required: true,
			},
		},
	],
};
