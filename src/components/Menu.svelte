<svg on:click={()=>showMenu=!showMenu} xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M24 13.616v-3.232c-1.651-.587-2.694-.752-3.219-2.019v-.001c-.527-1.271.1-2.134.847-3.707l-2.285-2.285c-1.561.742-2.433 1.375-3.707.847h-.001c-1.269-.526-1.435-1.576-2.019-3.219h-3.232c-.582 1.635-.749 2.692-2.019 3.219h-.001c-1.271.528-2.132-.098-3.707-.847l-2.285 2.285c.745 1.568 1.375 2.434.847 3.707-.527 1.271-1.584 1.438-3.219 2.02v3.232c1.632.58 2.692.749 3.219 2.019.53 1.282-.114 2.166-.847 3.707l2.285 2.286c1.562-.743 2.434-1.375 3.707-.847h.001c1.27.526 1.436 1.579 2.019 3.219h3.232c.582-1.636.75-2.69 2.027-3.222h.001c1.262-.524 2.12.101 3.698.851l2.285-2.286c-.744-1.563-1.375-2.433-.848-3.706.527-1.271 1.588-1.44 3.221-2.021zm-12 2.384c-2.209 0-4-1.791-4-4s1.791-4 4-4 4 1.791 4 4-1.791 4-4 4z"/></svg>

{#if showMenu}
<div class="menu">
   <div class="menu-item proxies" bind:this={proxy_div} >
    <span>Use Proxies</span>
    <button on:click={()=>$use_proxy=!$use_proxy}>Yes</button>
    <button on:click={()=>$use_proxy=!$use_proxy} class="active">No</button>
   </div>

   <div class=" menu-item change_latency">
    <span>Change Latency Min|Max</span>
    <input bind:value={$low_ms} type="text" placeholder="lowest">
    <input bind:value={$high_ms} type="text" placeholder="highest">
   </div>
{#if $use_proxy}
   <div class=" menu-item proxy_list">
    <p>Enter Proxy List Each Line</p>
    <textarea bind:value={textarea_value}></textarea>
    <button>Test Proxies</button>
   </div>
   {/if }

</div>
{/if}









<script lang="ts">
    import {use_proxy,low_ms,high_ms} from '@src/store/store';
import { invoke } from '@tauri-apps/api/tauri';
   let showMenu=false;
   let proxy_div:HTMLDivElement;
   let textarea_value:string;
$:{
    if(!proxy_div)break $;
   for (let button of proxy_div.querySelectorAll("button")){
    button.classList.remove("active")
   }
    if ($use_proxy){
    proxy_div.querySelectorAll("button")[0].classList.add("active")
}else{
    proxy_div.querySelectorAll("button")[1].classList.add("active")
}
    }
$:{
    if($use_proxy){
        invoke("get_proxies_from_file").then((data:any)=>textarea_value=data)
    }
}

</script>


<style>
    svg{
        position: absolute;
        top:0;
        right:0;
        margin:20px
    }
    .menu{
        display: flex;
    flex-direction: column;
        position: absolute;
        left:10%;
        top:10%;
        width:80%;
        height:80%;
        background: white;
        border:1px solid
    }
   .menu-item span{
    margin-right: auto;
    font-size: 25px;
   }
   .menu-item{
    display:flex;
    align-items: center;

    padding:10px 20px;
    border-bottom: 1px solid;
   }
   .proxies button,.proxy_list button{
    
    padding:10px 20px;
    font-size: 20px;
    border:1px solid;
   }
   .proxies button:first-of-type{
    border-top-left-radius: 12px;
    border-bottom-left-radius: 12px;
   }
   .proxies button:last-child{
    border-top-right-radius: 12px;
    border-bottom-right-radius: 12px;
   }
   .active{
    background-color: green;
    color:white;
   }
   .change_latency input{
        padding:10px 0;
        font-size:25px;
        width:100px;
        text-align: center;
        outline: none;
        border:1px solid
   }
   .change_latency input:first-of-type{
    border-top-left-radius: 12px;
    border-bottom-left-radius: 12px;
   }
   .change_latency input:last-child{
    border-top-right-radius: 12px;
    border-bottom-right-radius: 12px;
   }
   .proxy_list{
    flex-grow: 1;
    flex-direction: column;
   }
   .proxy_list textarea{
    width:100%;
    height:100%;
    resize: none;
    outline: none;
    font-size:20px;
   }
   .proxy_list p{
    font-size: 25px;
    padding:10px 0
   }
   .proxy_list button{
    margin-top:15px;
   }
   button:active{
    transform: scale(0.9)
   }
</style>