<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { ResultTable } from "$lib/components";

  const { Story } = defineMeta({
    title: "Components/ResultTable",
    component: ResultTable,
    tags: ["autodocs"],
    args: {
      message: "No results to display yet.",
    },
    argTypes: {
      message: {
        control: "text",
      },
    },
  });
</script>

<Story name="Empty" />

<Story
  name="Manifest not generated"
  args={{
    message:
      "No manifest results are available yet. Create an integrity record first.",
  }}
/>

<Story
  name="Check not run"
  args={{
    message:
      "No integrity check results are available yet. Run a backup check to see results.",
  }}
/>

<Story
  name="Filtered empty state"
  args={{
    message: "No files match the current report filter.",
  }}
/>
