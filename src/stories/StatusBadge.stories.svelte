<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { StatusBadge } from "$lib/components";

  const { Story } = defineMeta({
    title: "Componets/StatusBadge",
    component: StatusBadge,
    tags: ["autodocs"],
    argTypes: {
      label: {
        control: { type: "text" },
      },
      tone: {
        control: { type: "select" },
        options: ["neutral", "success", "warning", "danger", "info"],
        defaultValue: "neutral",
      },
    },
  });
</script>

<Story name="Neutral" args={{ label: "Neutral" }} />
<Story name="Success" args={{ label: "Healthy", tone: "success" }} />
<Story name="Warning" args={{ label: "Warning status", tone: "warning" }} />
<Story name="Danger" args={{ label: "Danger status", tone: "danger" }} />
<Story name="Info" args={{ label: "Info message", tone: "info" }} />

<Story name="AllTones">
  <div class="tones-row">
    <StatusBadge label="Neutral" />
    <StatusBadge label="Healthy" tone="success" />
    <StatusBadge label="Warning" tone="warning" />
    <StatusBadge label="Danger" tone="danger" />
    <StatusBadge label="Info" tone="info" />
  </div>
</Story>

<style>
  .tones-row {
    display: flex;
    gap: 0.75rem;
    flex-wrap: wrap;
    align-items: center;
  }
</style>
