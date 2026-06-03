CREATE INDEX idx_scans_project_kind_status_started
ON scans(project_id, kind, status, started_at);

CREATE INDEX idx_manifest_project_seen
ON manifest_entries(project_id, last_seen_scan_id);
