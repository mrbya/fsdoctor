<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { PageHeader } from "$lib/components";

  const { Story } = defineMeta({
    title: "Components/PageHeader",
    component: PageHeader,
    tags: ["autodocs"],
    argTypes: {
      title: {
        control: { type: "text" },
      },
      eyebrow: {
        control: { type: "text" },
        defaultValue: "FsDoctor",
      },
      description: {
        control: { type: "text" },
      },
    },
  });
</script>

<Story name="Simple" args={{ title: "View header" }} />
<Story
  name="WithDescription"
  args={{
    title: "Create integrity record",
    description: "Create an FSDoctor project database for a backup folder.",
  }}
/>
<Story
  name="AltEyebrow"
  args={{
    title: "Settings",
    eyebrow: "Developer mode",
    description: "Developer settings.",
  }}
/>
