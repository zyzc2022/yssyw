<template>
    <div class="root" @click="$emit('click')">
        <img v-if="back !== ''" :src="back" >
        <span class="s1"></span>
        <span class="s2"></span>
        <span class="s3"></span>
        <span class="s4"></span>
        +
    </div>
</template>

<script>
export default {
    name: "AddButton",
    props: {
        back: { default: "" }
    }
}
</script>

<style lang="scss" scoped>
$color: rgb(217, 236, 255);
$hoverColor: rgb(217, 236, 255);
$border: 2px solid $color;
$border-radius: 10px;

$backSize: 58px;

.root {
    position: relative;
    //width: 10vw;
    //height: 5vw;
    // background: red;
    display: inline-flex;
    justify-content: center;
    align-items: center;
    font-size: 20px;
    color: $color;
    cursor: pointer;
    border-radius: $border-radius;

    img {
        position: absolute;
        height: $backSize;
        width: $backSize;
        opacity: 0.5;
    }

    span {
        height: 20px;
        width: 20px;
        position: absolute;
    }

    &:hover {
        background: #00000005;

        span {
            border-color: $hoverColor;
        }
    }

    .s1 {
        left: 0;
        top: 0;
        border-left: $border;
        border-top: $border;
        //border-top-left-radius: $border-radius;
    }

    .s2 {
        top: 0;
        right: 0;
        border-top: $border;
        border-right: $border;
        //border-top-right-radius: $border-radius;
    }

    .s3 {
        bottom: 0;
        right: 0;
        border-bottom: $border;
        border-right: $border;
        //border-bottom-right-radius: $border-radius;
    }

    .s4 {
        bottom: 0;
        left: 0;
        border-bottom: $border;
        border-left: $border;
        //border-bottom-left-radius: $border-radius;
    }
}
</style>