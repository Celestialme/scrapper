<script lang='ts'>
import { onMount,onDestroy } from "svelte";
import { listen } from "@tauri-apps/api/event";


    export let value:number;
    export let max_value:number;
    export let keyword:string;
    let unlisten:Function;
    let unlisten2:Function;
    let proxy_info=""
    let sleep_info=""
    $:percent = Math.round(value/(max_value || 1) * 100)
    onMount(() => {
        listen('proxy_info', (e:any) => {
            proxy_info = e.payload;
        }).then(_unlisten=>unlisten=_unlisten)


        listen('sleep_info', (e:any) => {
            sleep_info = e.payload;
        }).then(_unlisten=>unlisten2=_unlisten)
    })
    onDestroy(()=>{
        unlisten?.();
        unlisten2?.();
    })
    </script>
    
    <div style='text-align:left;padding:20px 0px;width: 70%' >
        <div style="display:flex">
            <span class='msg'>{@html keyword}</span>
            
            <span class="percent">{percent}%</span>
        </div>
        <div class='progress-track' >
            <div class='progress-bar'   style='width:{percent}%'></div>
        </div>
        <p class='proxy_info'>{@html proxy_info}</p>
        <p class='sleep_info'>{@html sleep_info}</p>
    </div>
    
    <style>
    
        
        .progress-track{
            height: 10px;
            background: #e0e0e0;
            border-radius: 10px;
            margin-top: 15px;
        }
        .progress-bar{
            width:0;
            height: 100%;
            background: #00bcd4;
            border-radius: 10px;
            transition:1s;
        }
      
        .percent{
            font-size:20px;
            margin-left:auto
        }
      .msg{
        font-size:20px;
        margin-right: auto;
      }
      .proxy_info{
        margin-top: 15px;
        text-align:center;
        font-size: 20px;
      }
      .sleep_info{
        position: absolute;
        bottom:0;
        left:0;
        text-align:center;
        width:100%;
        font-size: 20px;
      }
    </style>