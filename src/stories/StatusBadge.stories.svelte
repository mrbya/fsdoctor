<script lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { StatusBadge } from "$lib/components";

  const { Story } = defineMeta({
    title: "Componnets/StatusBadge",
    component: StatusBadge,
    tags: ["autodocs"],
    argTypes: {
      tone: {
        control: { type: "select" },
        options: ["neutral", "success", "warning", "danger", "info"],
      },
    },
  });
</script>

<Story name="Neutral">
  <StatusBadge label="Neutral" />
</Story>

<Story name="Success">
  <StatusBadge label="Healthy" tone="success" />
</Story>

<Story name="Warning">
  <StatusBadge label="Warning status" tone="warning" />
</Story>

<Story name="Danger">
  <StatusBadge label="Danger status" tone="danger" />
</Story>

<Story name="Info">
  <StatusBadge label="Info status" tone="info" />
</Story>
