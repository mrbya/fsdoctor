<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { Card } from "$lib/components";

  const { Story } = defineMeta({
    title: "Components/Card",
    component: Card,
    tags: ["autodocs"],
    argTypes: {
      tone: {
        control: { type: "select" },
        options: ["default", "success", "warning", "danger", "info"],
      },
    },
  });
</script>

<Story name="Default">
  <Card>
    <p>This is a default card</p>
  </Card>
</Story>

<Story name="Success">
  <Card tone="success">
    <p>This is a success card</p>
  </Card>
</Story>

<Story name="Warning">
  <Card tone="warning">
    <p>This is a warning card</p>
  </Card>
</Story>

<Story name="Danger">
  <Card tone="danger">
    <p>This is a danger card</p>
  </Card>
</Story>

<Story name="Info">
  <Card tone="info">
    <p>This is a info card</p>
  </Card>
</Story>

<Story name="WithComplexContent">
  <Card>
    <table class="entry-table">
      <thead>
        <tr>
          <th>File</th>
          <th>Size</th>
          <th>Created</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td>/test/file.1</td>
          <td>23MiB</td>
          <td>2026-03-24</td>
        </tr>
        <tr>
          <td>/test/file.2</td>
          <td>105MiB</td>
          <td>2026-03-23</td>
        </tr>
        <tr>
          <td>/test/file.3</td>
          <td>3GiB</td>
          <td>2026-03-22</td>
        </tr>
      </tbody>
    </table>
  </Card>
</Story>

<Story name="AllTones">
  <div class="tones-row">
    <Card>
      <p>default</p>
    </Card>
    <Card tone="success">
      <p>success</p>
    </Card>
    <Card tone="warning">
      <p>warning</p>
    </Card>
    <Card tone="danger">
      <p>danger</p>
    </Card>
    <Card tone="info">
      <p>info</p>
    </Card>
  </div>
</Story>

<style>
  .entry-table {
    width: 100%;
    border-collapse: collapse;
    font-size: var(--font-size-sm);
    color: var(--text);
  }

  .entry-table th {
    text-align: left;
    padding: 0.3rem 0.5rem;
    color: var(--text-muted);
    font-weight: 500;
    border-bottom: 1px solid var(--border);
  }

  .entry-table td {
    padding: 0.4rem 0.5rem;
    border-bottom: 1px solid var(--border);
  }

  .entry-table tr:last-child td {
    border-bottom: none;
  }

  .tones-row {
    display: flex;
    gap: 0.75rem;
    flex-wrap: wrap;
    align-items: center;
  }
</style>
