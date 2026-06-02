<script module lang="ts">
  import { fn } from "storybook/test";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { FilePickerRow } from "$lib/components";

  const { Story } = defineMeta({
    title: "Components/FilePickerRow",
    component: FilePickerRow,
    tags: ["autodocs"],
    args: {
      label: "Backup root path",
      value: "",
      placeholder: "D:\\Backups\\OldShare",
      buttonLabel: "Browse",
      disabled: false,
      onbrowse: fn(),
    },
    argTypes: {
      label: { control: "text" },
      value: { control: "text" },
      placeholder: { control: "text" },
      buttonLabel: { control: "text" },
      disabled: { control: "boolean" },
    },
  });
</script>

<Story name="Empty" />

<Story
  name="With value"
  args={{
    value: "D:\\Backups\\OldShare",
  }}
/>

<Story
  name="Database path"
  args={{
    label: "FSDoctor database path",
    value: "C:\\Users\\Viktor\\Documents\\OldShare.fsdoctor.sqlite",
    placeholder: "C:\\Users\\User\\Documents\\OldShare.fsdoctor.sqlite",
    buttonLabel: "Select file",
  }}
/>

<Story
  name="Disabled"
  args={{
    value: "D:\\Backups\\OldShare",
    disabled: true,
  }}
/>
