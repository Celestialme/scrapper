<div class="body">
  <ThemeIcon/>
  <Menu/>
  <ProgressBar {value} {max_value} {keyword} />
  {#if !crawler_ready}
   <label  for="file"> Upload Text File</label> <input  type="file" id='file' hidden bind:files={files} on:change={read_file}>
   {/if}
   {#if crawler_ready} 
   <button on:click={start}>Start Crawling</button>
   {/if}
</div>


<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from "@tauri-apps/api/event";
import { onMount,onDestroy } from 'svelte';
import ProgressBar from '@src/components/ProgressBar.svelte';
import Menu from '@src/components/Menu.svelte';
import {max_ms, min_ms, proxy_list} from '@src/store/store';
import ThemeIcon from '@src/components/ThemeIcon.svelte';
let unlisten:Function;
let unlisten2:Function;
let crawler_ready=false;
let files:any;
let reader:any;
let keyword="";
let max_value=1;
let value=0;
function read_file(){
keyword="";
max_value=1;
value=0;
  reader.readAsText(files[0])
}


onMount(()=>{
  reader = new  FileReader();
  listen('parse_html', (e:any) => {
    parse_html(e.payload);
}).then(_unlisten=>unlisten=_unlisten)


listen('progress', (e:any) => {
    let data = JSON.parse(e.payload)
    keyword=data.keyword
    value=data.progress.count*1;
    max_value=data.progress.maxValue*1;
    if (value == max_value) {
      crawler_ready=false;
    }
}).then(_unlisten=>unlisten=_unlisten)

reader.onload = function(){
  
  crawler_ready = true;
}


})

onDestroy(()=>{
  unlisten?.();
  unlisten2?.();
})


function start(){
  let keywords = reader.result.trim().split(/\r\n|\n/g).filter((x:any)=>x!="")
  
  invoke("start_crawling",{keywords,proxies:$proxy_list.trim().split(/\r\n|\n/g).filter((x:any)=>x!=""),sleep:{min:$min_ms*1,max:$max_ms*1}})
}


async function parse_html(html:string){
  
  let _div:HTMLDivElement = document.createElement('div');
  _div.innerHTML = html;
  let results = [...[..._div.querySelectorAll("div.g a h3")].map((res:any)=>res?.parentElement.href),
                 ...[..._div.querySelectorAll("div.g h3 a")].map((res:any)=>res.href)]
  console.log(results)
  invoke("append_urls_to_file",{list:results})
}
   
</script>
<style>
  :global(*){
    margin:0;
    padding:0;
    box-sizing:border-box;
    
    
  }
  :global(.body,.menu){
    transition: background-color 0.3s ease-in-out;
  }
  :global(html){
    --text-color: black;
    --background-color:white;
  }
  :global(html.dark_theme){
    --text-color: white;
    --background-color:#313030;
  }
.body {
  height: 100vh;
  width: 100vw;
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
  background-color: var(--background-color);
  color: var(--text-color);
  fill: var(--text-color);
}
label,button{
  width:300px;
  text-align: center;
  font-size: 23px;
    border: 1px solid;
    padding: 15px;
    border-radius: 10px;
    font-family: monospace;
    background-color:#00f7ff;
    color:black;
}
:global( button:active,label:active,svg:active){
    transform: scale(0.9)
   }
   :global( button:hover,label:hover){
    background-color: #8afdfd;
   }
</style>