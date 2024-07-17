import { client, type ModelDefinition } from "filigree-svelte";
import { z } from "zod";
import { ObjectPermission } from "../model_types.js";
import {
	ReportSectionCreatePayloadSchema,
	ReportSectionSchema,
	ReportSectionUpdatePayloadSchema,
} from "./report_section.js";
import {
	TagCreatePayloadSchema,
	TagSchema,
	TagUpdatePayloadSchema,
} from "./tag.js";

export type ReportId = string;

export const ReportSchema = z.object({
	id: z.string(),
	organization_id: z.string(),
	updated_at: z.string().datetime(),
	created_at: z.string().datetime(),
	title: z.string(),
	description: z.string().optional(),
	ui: z.any(),
});

export type Report = z.infer<typeof ReportSchema>;
export const ReportListResultSchema = ReportSchema;
export type ReportListResult = Report;

export const ReportCreatePayloadSchema = z.object({
	id: z.string().optional(),
	title: z.string(),
	description: z.string().optional(),
	ui: z.any(),
	report_sections: ReportSectionCreatePayloadSchema.array().optional(),
	tags: z.string().array().optional(),
});

export type ReportCreatePayload = z.infer<typeof ReportCreatePayloadSchema>;

export const ReportCreateResultSchema = z.object({
	id: z.string(),
	organization_id: z.string(),
	updated_at: z.string().datetime(),
	created_at: z.string().datetime(),
	title: z.string(),
	description: z.string().optional(),
	ui: z.any(),
	report_sections: ReportSectionCreatePayloadSchema.array(),
	tags: z.string().array(),
});

export type ReportCreateResult = z.infer<typeof ReportCreateResultSchema>;

export const ReportPopulatedGetResultSchema = z.object({
	id: z.string(),
	organization_id: z.string(),
	updated_at: z.string().datetime(),
	created_at: z.string().datetime(),
	title: z.string(),
	description: z.string().optional(),
	ui: z.any(),
	report_sections: ReportSectionSchema.array(),
	tags: TagSchema.array(),
});

export type ReportPopulatedGetResult = z.infer<
	typeof ReportPopulatedGetResultSchema
>;

export const ReportPopulatedListResultSchema = z.object({
	id: z.string(),
	organization_id: z.string(),
	updated_at: z.string().datetime(),
	created_at: z.string().datetime(),
	title: z.string(),
	description: z.string().optional(),
	ui: z.any(),
	report_section_ids: z.string().array(),
	tag_ids: z.string().array(),
});

export type ReportPopulatedListResult = z.infer<
	typeof ReportPopulatedListResultSchema
>;

export const ReportUpdatePayloadSchema = z.object({
	id: z.string().optional(),
	title: z.string(),
	description: z.string().optional(),
	ui: z.any(),
	report_sections: ReportSectionUpdatePayloadSchema.array().optional(),
	tags: z.string().array().optional(),
});

export type ReportUpdatePayload = z.infer<typeof ReportUpdatePayloadSchema>;

export const baseUrl = "reports";
export const urlWithId = (id: string) => `${baseUrl}/${id}`;

export const urls = {
	create: baseUrl,
	list: baseUrl,
	get: urlWithId,
	update: urlWithId,
	delete: urlWithId,
};

export const ReportModel: ModelDefinition<typeof ReportSchema> = {
	name: "Report",
	plural: "Reports",
	baseUrl,
	urls,
	schema: ReportSchema,
	createSchema: ReportCreatePayloadSchema,
	updateSchema: ReportUpdatePayloadSchema,
	fields: [
		{
			name: "id",
			type: "uuid",
			label: "Id",
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
		{
			name: "title",
			type: "text",
			label: "Title",
			constraints: {
				required: true,
			},
		},
		{
			name: "description",
			type: "text",
			label: "Description",
			constraints: {
				required: false,
			},
		},
		{
			name: "ui",
			type: "object",
			label: "Ui",
			constraints: {
				required: true,
			},
		},
	],
};
