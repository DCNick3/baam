<script lang=ts>
    export let text:string;
    export let link:string;
    export let icon:string;
</script>

<a href={link} class="m-4 ml-0 text-base text-slate-900 hover:text-slate-700 hidden lg:inline-block"> {text} </a>