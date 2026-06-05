<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { ProgressPanel } from "$lib/components";

  const { Story } = defineMeta({
    title: "Components/ProgressPanel",
    component: ProgressPanel,
    tags: ["autodocs"],
    argTypes: {
      eyebrow: {
        control: { type: "text" },
      },
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
  name="WithEyebrow"
  args={{
    eyebrow: "Integrity check",
    title: "Scanning and hashing files",
    description:
      "FSDoctor is checking the current backup tree and keeping the latest counters visible.",
  }}
/>
