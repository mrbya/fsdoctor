<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { MetricGrid } from "$lib/components";

  const { Story } = defineMeta({
    title: "Components/MetricGrid",
    component: MetricGrid,
    tags: ["autodocs"],
  });

  const manifestMetrics = [
    { label: "Files seen", value: "42,118" },
    { label: "Directories seen", value: "1,203" },
    { label: "Data discovered", value: "418 GiB" },
    { label: "Files hashed", value: "40,991" },
    { label: "Unreadable entries", value: "2", tone: "warning" as const },
    { label: "Changed during scan", value: "1", tone: "warning" as const },
  ];

  const reportMetrics = [
    { label: "OK", value: "41,981", tone: "success" as const },
    { label: "Missing", value: "3", tone: "danger" as const },
    { label: "New", value: "8", tone: "warning" as const },
    { label: "Hash mismatch", value: "1", tone: "danger" as const },
  ];
</script>

<Story name="Manifest progress">
  <MetricGrid items={manifestMetrics} />
</Story>

<Story name="Final report summary">
  <MetricGrid items={reportMetrics} />
</Story>
