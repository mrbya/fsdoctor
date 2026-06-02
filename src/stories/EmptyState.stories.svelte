<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { EmptyState } from "$lib/components";

  const { Story } = defineMeta({
    title: "Components/EmptyState",
    component: EmptyState,
    tags: ["autodocs"],
    argTypes: {
      title: {
        control: { type: "text" },
      },
      description: {
        control: { type: "text" },
        defaultValue: null,
      },
    },
  });
</script>

<Story name="Simple" args={{ title: "No project database." }} />
<Story
  name="WithDescription"
  args={{
    title: "No reports yet",
    description:
      "Reports will appear here after manifest generation and integrity checks are implemented.",
  }}
/>
