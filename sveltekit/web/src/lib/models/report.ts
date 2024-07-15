import { client, type ModelDefinition } from "filigree-svelte";
import { z } from "zod";
import { ObjectPermission } from "../model_types.js";
import {
	ReportSectionCreatePayloadSchema,
	ReportSectionSchema,
	ReportSectionUpdatePayloadSchema,
} from "./report_section.js";

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
export const ReportUserViewSchema = ReportSchema;
export type ReportUserView = Report;
export const ReportOwnerViewSchema = ReportSchema;
export type ReportOwnerView = Report;
export const ReportListResultSchema = ReportSchema;
export type ReportListResult = Report;
export const ReportSchema = ReportSchema;
export type Report = Report;

export const ReportCreateResultSchema = z.object({
	id: z.string(),
	organization_id: z.string(),
	updated_at: z.string().datetime(),
	created_at: z.string().datetime(),
	title: z.string(),
	description: z.string().optional(),
	ui: z.any(),
	report_sections: ReportSectionCreatePayloadSchema.array(),
});

export type ReportCreateResult = z.infer<typeof ReportCreateResultSchema>;

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
