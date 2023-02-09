<script lang="ts">
  import type Scene from '$lib/scene';
  import selectedScenes from '$lib/testdata';

  const emptyScene: Scene = {
    id: -1,
    fileName: '',
    directory: '',
    tags: '',
    numGirls: -1,
    numBoys: -1,
    score: 0
  };

  let selected = -1;

  let selection = () => (selected > -1 ? selectedScenes[selected] : emptyScene);
</script>

<div style="width:30%; float:left; height: 100%; overflow-y: scroll;">
  <ul>
    {#each selectedScenes as scene, index}
      <li>
        <input
          type="radio"
          name="scenelist"
          value={index}
          id="sc{scene.id}"
          bind:group={selected}
        /><label for="sc{scene.id}">{scene.name || scene.fileName}</label>
      </li>
    {/each}
  </ul>
</div>
<div style="width:70%; float:right;">
  {#if selected > -1}
    <img src={`https://thumbnail../?id=${selection().id}`} alt="Thumbnail" />
  {/if}
  <div>
    <p>fileName: {selection().fileName}</p>
    <p>Path: {selection().directory}</p>
    <!-- TODO: absolute path including file name -->
    <p>Year: {selection().year || '(unknown)'}</p>
    <p>WebSite: {selection().webSite || '(unknown)'}</p>
  </div>
</div>

<style>
  img {
    max-width: 100%;
  }
  li {
    list-style: none;
  }
  input[type='radio'] {
    appearance: none;
    -webkit-appearance: none;
  }
  input:checked + label {
    background: #888;
  }
</style>
