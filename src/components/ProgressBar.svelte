<script lang='ts'>
    export let value:number;
    export let max_value:number;
    export let keyword:string;
    $:console.log(value,max_value)
    $:percent = Math.round(value/(max_value || 1) * 100)
    
    </script>
    
    <div style='text-align:left;padding:20px 0px;width: 70%' >
        <span class="percent">{percent}%</span>
        <span class='msg'>{@html keyword}</span>
        <div class='progress-track' >
            <div class='progress-bar'   style='width:{percent}%'></div>
        </div>
        
    </div>
    
    <style>
        .wide{
            width:calc(80vw - 300px)
        }
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
            float:right
        }
      
    </style>