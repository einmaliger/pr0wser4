<script lang="ts">
  import { realLength, type Scene } from '$lib/scenedatabase';

  export let base_dir: string;
  export let selection: Scene;
  export let tags: string[][];

  interface TagClass {
    name: string;
    tags: string[];
  }

  let allTags: TagClass[] = [];

  //[1].split(",").filter(v => selection.tags.includes(v))

  // update alllTags
  $: {
    for (let cls of tags) {
      let tagList = cls[1].split(',').map((s) => s.trim());
      allTags.push({ name: cls[0], tags: tagList });
    }
  }

  let myTags: TagClass[] = [];

  // update mytags
  $: {
    let remainingTags = new Set(selection.tags);
    myTags = [];
    for (let cls of allTags) {
      let result: string[] = [];
      // find intersection
      for (let tag of cls.tags) {
        if (remainingTags.delete(tag)) {
          result.push(tag);
        }
      }
      if (result.length > 0) {
        myTags.push({
          name: cls.name,
          tags: result
        });
      }
    }
    if (remainingTags.size > 0) {
      myTags.push({
        name: '<unknown>',
        tags: Array.from(remainingTags)
      });
    }
  }
  // Return displayable version of num_girls / num_boys
  function numX(n: number): string | number {
    switch (n) {
      case -1:
        return 'unknown';
      case 9999:
        return 'many';
      default:
        return n;
    }
  }

  function displayLength(s: Scene): string {
    const length = realLength(s);
    const seconds = length % 60;
    const minutes = ((length - seconds) / 60) % 60;
    const hours = Math.floor(length / 3600);

    // There should be a more elegant way to format this
    let result = seconds.toString();
    if (seconds < 10) result = '0' + result;
    result = minutes.toString() + ':' + result;
    if (minutes < 10) result = '0' + result;
    if (hours > 0) result = hours.toString() + ':' + result;

    return result;
  }
</script>

<img
  src={`https://screenshot../?base_dir=${base_dir}&directory=${selection.directory}&${
    selection.thumb_file_name
      ? 'thumb_file_name=' + selection.thumb_file_name
      : 'file_name=' + selection.file_name
  }`}
  alt="Thumbnail"
/>
<div style="display: flex;">
  <div>{selection.name || selection.file_name} ({displayLength(selection)})</div>
  <div style="margin-left: auto;">
    {#if selection.score >= 20}<img src="star.svg" width="16" height="16" alt="*" />{/if}
    {#if selection.score >= 40}<img src="star.svg" width="16" height="16" alt="*" />{/if}
    {#if selection.score >= 60}<img src="star.svg" width="16" height="16" alt="*" />{/if}
    {#if selection.score >= 75}<img src="star.svg" width="16" height="16" alt="*" />{/if}
    {#if selection.score >= 90}<img src="star.svg" width="16" height="16" alt="*" />{/if}
    ({selection.score})
  </div>
</div>
<table>
  <tr>
    <td>File name:</td>
    <td>{selection.file_name}</td>
  </tr>
  <tr>
    <td>Path:</td>
    <td>{(base_dir || '') + '/' + selection.directory}</td>
  </tr>
  <tr>
    <td>Year:</td>
    <td>{selection.year || '(unknown)'}</td>
  </tr>
  <tr>
    <td>Website:</td>
    <td>{selection.website || '(unknown)'}</td>
  </tr>
  <tr>
    <td>Featuring:</td>
    <td>{selection.actors || '(unknown)'}</td>
  </tr>
  <tr>
    <td>Girls/Boys:</td>
    <td>
      {numX(selection.num_girls)} /
      {numX(selection.num_boys)}
    </td>
  </tr>
  {#if tags.length === 0}
    <tr>
      <td>Tags:</td>
      <td>{selection.tags.join(', ')}</td>
    </tr>
  {:else}
    <tr>
      <td>Tags</td>
      <td>
        <table>
          {#each myTags as tag}
            <tr>
              <td>{tag.name}</td>
              <td>{tag.tags.join(', ')}</td>
            </tr>
          {/each}
        </table>
      </td>
    </tr>
  {/if}

  {#if selection.notes}
    <tr>
      <td>Notes:</td>
      <td>{selection.notes}</td>
    </tr>
  {/if}
</table>

<style>
  img {
    max-width: 100%;
  }
</style>
