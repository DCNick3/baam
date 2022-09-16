{{- define "baam.postgresql.fullname" -}}
{{- printf "%s-%s" .Release.Name "postgresql" | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "baam.databaseHost" }}
{{- printf "%s" (include "baam.postgresql.fullname" .) -}}
{{- end }}

{{- define "baam.databasePort" -}}
{{- printf "5432" -}}
{{- end -}}

{{- define "baam.databaseName" -}}
{{- printf "%s" .Values.postgresql.auth.database -}}
{{- end -}}

{{- define "baam.databaseUser" -}}
{{- printf "%s" .Values.postgresql.auth.username -}}
{{- end -}}

{{- define "baam.databaseSecretName" -}}
{{- printf "%s" .Values.postgresql.auth.existingSecret -}}
{{- end -}}