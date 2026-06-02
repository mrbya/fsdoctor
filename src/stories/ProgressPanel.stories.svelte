<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { ProgressPanel } from "$lib/components";

  const { Story } = defineMeta({
    title: "Components/ProgressPanel",
    component: ProgressPanel,
    tags: ["autodocs"],
    argTypes: {
      title: {
        control: { type: "text" },
      },
      description: {
        control: { type: "text" },
      },
    },
  });
</script>

<Story name="Default" />
<Story name="Simple" args={{ title: "Updating..." }} />
<Story
  name="WithDescription"
  args={{
    title: "No check running",
    description:
      "Progress will appear here once integrity checks are implemented.",
  }}
/>
