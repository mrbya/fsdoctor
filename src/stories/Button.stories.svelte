<script module lang="ts">
  import { Button } from "$lib/components";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { fn } from "storybook/test";

  const onClick = fn();

  const { Story } = defineMeta({
    title: "Components/Button",
    component: Button,
    tags: ["autodocs"],
    argTypes: {
      variant: {
        control: { type: "select" },
        options: ["primary", "secondary", "ghost", "danger"],
      },
    },
  });
</script>

<Story name="Primary">
  <Button variant="primary" onclick={onClick}>Check integrity</Button>
</Story>

<Story name="Secondary">
  <Button variant="secondary" onclick={onClick}>Start</Button>
</Story>

<Story name="Ghost">
  <Button variant="ghost" onclick={onClick}>Cancel</Button>
</Story>

<Story name="Danger">
  <Button variant="danger" onclick={onClick}>Delete project</Button>
</Story>

<Story name="AllVariants">
  <div class="variants-row">
    <Button variant="primary" onclick={onClick}>Check integrity</Button>
    <Button variant="secondary" onclick={onClick}>Start</Button>
    <Button variant="ghost" onclick={onClick}>Cancel</Button>
    <Button variant="danger" onclick={onClick}>Delete project</Button>
  </div>
</Story>

<style>
  .variants-row {
    display: flex;
    gap: 0.75rem;
    flex-wrap: wrap;
    align-items: center;
  }
</style>
