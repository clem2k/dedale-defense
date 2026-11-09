# How it works

Deep mode: what the site's JavaScript is made of and what its start page did while we watched.

## Its JavaScript

Analysed 32 script(s) from https://www.kongregate.com/en/games/preecep/desktop-tower-defense: 21 module(s), 260 endpoint(s), 1 route(s).

Evidence ev_8b2ec460060697ed7d4f067558be8f2b1240d692298b08a659f8844184985261.

```json
{
  "api_operations": [],
  "app_kind": "unknown",
  "bundlers": [],
  "edges": [
    {
      "from": "event:analytics.ahrefs.com/analytics.js#document:pagehide",
      "kind": "handled-by",
      "to": "fn:analytics.ahrefs.com/analytics.js#document.addEventListener(pagehide)@1"
    },
    {
      "from": "event:analytics.ahrefs.com/analytics.js#document:visibilitychange",
      "kind": "handled-by",
      "to": "fn:analytics.ahrefs.com/analytics.js#document.addEventListener(visibilitychange)@1"
    },
    {
      "from": "event:analytics.ahrefs.com/analytics.js#r:readystatechange",
      "kind": "handled-by",
      "to": "fn:analytics.ahrefs.com/analytics.js#r.onreadystatechange@1"
    },
    {
      "from": "event:analytics.ahrefs.com/analytics.js#t:auxclick",
      "kind": "handled-by",
      "to": "fn:analytics.ahrefs.com/analytics.js#e@1"
    },
    {
      "from": "event:analytics.ahrefs.com/analytics.js#t:click",
      "kind": "handled-by",
      "to": "fn:analytics.ahrefs.com/analytics.js#e@1"
    },
    {
      "from": "event:analytics.ahrefs.com/analytics.js#t:submit",
      "kind": "handled-by",
      "to": "fn:analytics.ahrefs.com/analytics.js#e@1"
    },
    {
      "from": "event:analytics.ahrefs.com/analytics.js#u:readystatechange",
      "kind": "handled-by",
      "to": "fn:analytics.ahrefs.com/analytics.js#u.onreadystatechange@1"
    },
    {
      "from": "event:analytics.ahrefs.com/analytics.js#window.document:visibilitychange",
      "kind": "handled-by",
      "to": "fn:analytics.ahrefs.com/analytics.js#window.document.addEventListener(visibilitychange)@1"
    },
    {
      "from": "event:analytics.ahrefs.com/analytics.js#window:DOMContentLoaded",
      "kind": "handled-by",
      "to": "fn:analytics.ahrefs.com/analytics.js#window.addEventListener(DOMContentLoaded)@1"
    },
    {
      "from": "event:analytics.ahrefs.com/analytics.js#window:popstate",
      "kind": "handled-by",
      "to": "fn:analytics.ahrefs.com/analytics.js#h@1"
    },
    {
      "from": "event:cdn.jsdelivr.net/npm/videojs-contrib-ads@7.4.0/dist/videojs.ads.min.js#e:loadstart",
      "kind": "handled-by",
      "to": "fn:cdn.jsdelivr.net/npm/videojs-contrib-ads@7.4.0/dist/videojs.ads.min.js#e.on(loadstart)@2"
    },
    {
      "from": "event:cdn.jsdelivr.net/npm/videojs-contrib-ads@7.4.0/dist/videojs.ads.min.js#n.textTracks():change",
      "kind": "handled-by",
      "to": "fn:cdn.jsdelivr.net/npm/videojs-contrib-ads@7.4.0/dist/videojs.ads.min.js#i@2"
    },
    {
      "from": "event:cdn.jsdelivr.net/npm/videojs-contrib-ads@7.4.0/dist/videojs.ads.min.js#n:addtrack",
      "kind": "handled-by",
      "to": "fn:cdn.jsdelivr.net/npm/videojs-contrib-ads@7.4.0/dist/videojs.ads.min.js#n.addEventListener(addtrack)@2"
    },
    {
      "from": "event:cdn.jsdelivr.net/npm/videojs-contrib-ads@7.4.0/dist/videojs.ads.min.js#n:contenttimeupdate",
      "kind": "handled-by",
      "to": "fn:cdn.jsdelivr.net/npm/videojs-contrib-ads@7.4.0/dist/videojs.ads.min.js#n.on(contenttimeupdate)@2"
    },
    {
      "from": "event:cdn.jsdelivr.net/npm/videojs-contrib-ads@7.4.0/dist/videojs.ads.min.js#n:dispose",
      "kind": "handled-by",
      "to": "fn:cdn.jsdelivr.net/npm/videojs-contrib-ads@7.4.0/dist/videojs.ads.min.js#n.on(dispose)@2"
    },
    {
      "from": "event:cdn.jsdelivr.net/npm/videojs-contrib-ads@7.4.0/dist/videojs.ads.min.js#n:ended",
      "kind": "handled-by",
      "to": "fn:cdn.jsdelivr.net/npm/videojs-contrib-ads@7.4.0/dist/videojs.ads.min.js#n.on(ended)@2"
    },
    {
      "from": "event:cdn.jsdelivr.net/npm/videojs-contrib-ads@7.4.0/dist/videojs.ads.min.js#n:loadeddata",
      "kind": "handled-by",
      "to": "fn:cdn.jsdelivr.net/npm/videojs-contrib-ads@7.4.0/dist/videojs.ads.min.js#n.on(loadeddata)@2"
    },
    {
      "from": "event:cdn.jsdelivr.net/npm/videojs-contrib-ads@7.4.0/dist/videojs.ads.min.js#n:loadedmetadata",
      "kind": "handled-by",
      "to": "fn:cdn.jsdelivr.net/npm/videojs-contrib-ads@7.4.0/dist/videojs.ads.min.js#n.on(loadedmetadata)@2"
    },
    {
      "from": "event:cdn.jsdelivr.net/npm/videojs-contrib-ads@7.4.0/dist/videojs.ads.min.js#n:message",
      "kind": "handled-by",
      "to": "fn:cdn.jsdelivr.net/npm/videojs-contrib-ads@7.4.0/dist/videojs.ads.min.js#n.addEventListener(message)@2"
    },
    {
      "from": "event:cdn.jsdelivr.net/npm/videojs-contrib-ads@7.4.0/dist/videojs.ads.min.js#n:nopostroll",
      "kind": "handled-by",
      "to": "fn:cdn.jsdelivr.net/npm/videojs-contrib-ads@7.4.0/dist/videojs.ads.min.js#n.on(nopostroll)@2"
    },
    {
      "from": "event:cdn.jsdelivr.net/npm/videojs-contrib-ads@7.4.0/dist/videojs.ads.min.js#n:nopreroll",
      "kind": "handled-by",
      "to": "fn:cdn.jsdelivr.net/npm/videojs-contrib-ads@7.4.0/dist/videojs.ads.min.js#n.on(nopreroll)@2"
    },
    {
      "from": "event:cdn.jsdelivr.net/npm/videojs-contrib-ads@7.4.0/dist/videojs.ads.min.js#n:play",
      "kind": "handled-by",
      "to": "fn:cdn.jsdelivr.net/npm/videojs-contrib-ads@7.4.0/dist/videojs.ads.min.js#n.on(play)@2"
    },
    {
      "from": "event:cdn.jsdelivr.net/npm/videojs-contrib-ads@7.4.0/dist/videojs.ads.min.js#n:playing",
      "kind": "handled-by",
      "to": "fn:cdn.jsdelivr.net/npm/videojs-contrib-ads@7.4.0/dist/videojs.ads.min.js#n.on(playing)@2"
    },
    {
      "from": "event:cdn.jsdelivr.net/npm/videojs-contrib-ads@7.4.0/dist/videojs.ads.min.js#t:message",
      "kind": "handled-by",
      "to": "fn:cdn.jsdelivr.net/npm/videojs-contrib-ads@7.4.0/dist/videojs.ads.min.js#t.addEventListener(message)@2"
    },
    {
      "from": "event:cdn.jsdelivr.net/npm/videojs-contrib-ads@7.4.0/dist/videojs.ads.min.js#t:play",
      "kind": "handled-by",
      "to": "fn:cdn.jsdelivr.net/npm/videojs-contrib-ads@7.4.0/dist/videojs.ads.min.js#B@2"
    },
    {
      "from": "event:cdn.jsdelivr.net/npm/videojs-ima@2.1.0/dist/videojs.ima.min.js#i:cuechange",
      "kind": "handled-by",
      "to": "fn:cdn.jsdelivr.net/npm/videojs-ima@2.1.0/dist/videojs.ima.min.js#i.oncuechange@1"
    },
    {
      "from": "event:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#Hc():visibilitychange",
      "kind": "handled-by",
      "to": "fn:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#A@1"
    },
    {
      "from": "event:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#document:DOMContentLoaded",
      "kind": "handled-by",
      "to": "fn:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#document.addEventListener(DOMContentLoaded)@1"
    },
    {
      "from": "event:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#document:DOMContentLoaded",
      "kind": "handled-by",
      "to": "fn:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e@1"
    },
    {
      "from": "event:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#document:auth:open",
      "kind": "handled-by",
      "to": "fn:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#this.resetFormFocus@36"
    },
    {
      "from": "event:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#document:auth:open",
      "kind": "handled-by",
      "to": "fn:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#this.resetLoginFormFocus@35"
    },
    {
      "from": "event:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#document:auth:success",
      "kind": "handled-by",
      "to": "fn:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e@1"
    },
    {
      "from": "event:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#document:click",
      "kind": "handled-by",
      "to": "fn:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#document.addEventListener(click)@37"
    },
    {
      "from": "event:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#document:readystatechange",
      "kind": "handled-by",
      "to": "fn:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#this.interpretReadyState@20"
    },
    {
      "from": "event:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#document:submit",
      "kind": "handled-by",
      "to": "fn:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#document.addEventListener(submit)@40"
    },
    {
      "from": "event:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#document:turbo:before-visit",
      "kind": "handled-by",
      "to": "fn:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#this.willVisit@6"
    },
    {
      "from": "event:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#document:turbo:click",
      "kind": "handled-by",
      "to": "fn:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#this.linkClicked@6"
    },
    {
      "from": "event:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e.append(rect).attr(transform).attr(width).attr(height).attr(fill).on(mousemove):mouseleave",
      "kind": "handled-by",
      "to": "fn:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e.append(rect).attr(transform).attr(width).attr(height).attr(fill).on(mousemove).on(mouseleave)@38"
    },
    {
      "from": "event:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e.append(rect).attr(transform).attr(width).attr(height).attr(fill):mousemove",
      "kind": "handled-by",
      "to": "fn:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e.append(rect).attr(transform).attr(width).attr(height).attr(fill).on(mousemove)@38"
    },
    {
      "from": "event:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e.swiper.el:pointerenter",
      "kind": "handled-by",
      "to": "fn:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#L@1"
    },
    {
      "from": "event:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e.swiper.el:pointerleave",
      "kind": "handled-by",
      "to": "fn:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#_@1"
    },
    {
      "from": "event:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e.swiper.wrapperEl:transitionend",
      "kind": "handled-by",
      "to": "fn:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#t@1"
    },
    {
      "from": "event:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e.target:turbo:submit-start",
      "kind": "handled-by",
      "to": "fn:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e.target.addEventListener(turbo:submit-start)@1"
    },
    {
      "from": "event:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e:beforeTransitionStart",
      "kind": "handled-by",
      "to": "fn:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e.on(beforeTransitionStart)@5"
    },
    {
      "from": "event:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e:click",
      "kind": "handled-by",
      "to": "fn:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e.onclick@31"
    },
    {
      "from": "event:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e:destroy",
      "kind": "handled-by",
      "to": "fn:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#c@1"
    },
    {
      "from": "event:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e:destroy",
      "kind": "handled-by",
      "to": "fn:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e.on(destroy)@3"
    },
    {
      "from": "event:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e:error",
      "kind": "handled-by",
      "to": "fn:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#r@1"
    },
    {
      "from": "event:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e:impressionViewable",
      "kind": "handled-by",
      "to": "fn:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e.addEventListener(impressionViewable)@32"
    },
    {
      "from": "event:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e:init",
      "kind": "handled-by",
      "to": "fn:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e.on(init)@3"
    },
    {
      "from": "event:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e:init",
      "kind": "handled-by",
      "to": "fn:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#l@1"
    },
    {
      "from": "event:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e:input",
      "kind": "handled-by",
      "to": "fn:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e.addEventListener(input)@37"
    },
    {
      "from": "event:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e:keydown",
      "kind": "handled-by",
      "to": "fn:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e.addEventListener(keydown)@37"
    },
    {
      "from": "event:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e:load",
      "kind": "handled-by",
      "to": "fn:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e.addEventListener(load)@39"
    },
    {
      "from": "event:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e:load",
      "kind": "handled-by",
      "to": "fn:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#r@1"
    },
    {
      "from": "event:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e:message",
      "kind": "handled-by",
      "to": "fn:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#this.receiveMessageEvent@20"
    },
    {
      "from": "event:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e:slideChange",
      "kind": "handled-by",
      "to": "fn:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e.on(slideChange)@5"
    },
    {
      "from": "event:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e:sliderFirstMove",
      "kind": "handled-by",
      "to": "fn:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e.on(sliderFirstMove)@5"
    },
    {
      "from": "event:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e:slotRenderEnded",
      "kind": "handled-by",
      "to": "fn:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e.addEventListener(slotRenderEnded)@32"
    },
    {
      "from": "event:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e:touchEnd",
      "kind": "handled-by",
      "to": "fn:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e.on(touchEnd)@5"
    },
    {
      "from": "event:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#e:transitionend",
      "kind": "handled-by",
      "to": "fn:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#i@1"
    },
    {
      "from": "event:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#g.append(rect).attr(transform).attr(width).attr(height).attr(fill).on(mousemove):mouseleave",
      "kind": "handled-by",
      "to": "fn:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#g.append(rect).attr(transform).attr(width).attr(height).attr(fill).on(mousemove).on(mouseleave)@38"
    },
    {
      "from": "event:drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js#g.append(rect).attr(transform).attr(width).attr(height).attr(fill):mousemove",
      "kind": "handled-by",
      "to": "fn:drxf
```

## Ten seconds of it

Watched https://www.kongregate.com/en/games/preecep/desktop-tower-defense for 10000ms: 1 request outline(s), 25 console entr(y/ies).

Evidence ev_022dab3fea3cfa5e031eee1e17211844ff3e2c19fec25b15c9c45dd58466796c.

```json
{
  "accessibility": null,
  "console": {
    "count": 25,
    "entries": [
      {
        "level": "log",
        "source": "console-api"
      },
      {
        "level": "log",
        "source": "console-api"
      },
      {
        "level": "log",
        "source": "console-api"
      },
      {
        "level": "log",
        "source": "console-api"
      },
      {
        "level": "log",
        "source": "console-api"
      },
      {
        "level": "log",
        "source": "console-api"
      },
      {
        "level": "log",
        "source": "console-api"
      },
      {
        "level": "log",
        "source": "console-api"
      },
      {
        "level": "error",
        "source": "console-api"
      },
      {
        "level": "warning",
        "source": "console-api"
      },
      {
        "level": "log",
        "source": "console-api"
      },
      {
        "level": "log",
        "source": "console-api"
      },
      {
        "level": "log",
        "source": "console-api"
      },
      {
        "level": "error",
        "source": "console-api"
      },
      {
        "level": "error",
        "source": "console-api"
      },
      {
        "level": "log",
        "source": "console-api"
      },
      {
        "level": "log",
        "source": "console-api"
      },
      {
        "level": "log",
        "source": "console-api"
      },
      {
        "level": "log",
        "source": "console-api"
      },
      {
        "level": "log",
        "source": "console-api"
      },
      {
        "level": "log",
        "source": "console-api"
      },
      {
        "level": "warning",
        "source": "other"
      },
      {
        "level": "verbose",
        "source": "recommendation"
      },
      {
        "level": "warning",
        "source": "other"
      },
      {
        "level": "error",
        "source": "console-api"
      }
    ],
    "has_text": false
  },
  "dom": {
    "attribute_names": [
      "accept-charset",
      "action",
      "allow",
      "allowfullscreen",
      "alt",
      "aria-atomic",
      "aria-checked",
      "aria-describedby",
      "aria-disabled",
      "aria-expanded",
      "aria-haspopup",
      "aria-hidden",
      "aria-label",
      "aria-labelledby",
      "aria-live",
      "aria-modal",
      "aria-pressed",
      "aria-valuemax",
      "aria-valuemin",
      "aria-valuenow",
      "aria-valuetext",
      "async",
      "autocomplete",
      "charset",
      "checked",
      "class",
      "clip-rule",
      "content",
      "contents",
      "crossorigin",
      "cx",
      "cy",
      "d",
      "data-accounts-url",
      "data-action",
      "data-active-user-domain-value",
      "data-active-user-game-page-value",
      "data-active-user-single-cdn-domain-value",
      "data-active-user-trusted-domain-value",
      "data-active-user-user-data-value",
      "data-auth-modal-target",
      "data-auto-display",
      "data-callback",
      "data-comment-id",
      "data-comment-list",
      "data-components--text-tile-modal-target",
      "data-controller",
      "data-disable-with",
      "data-dislike-count",
      "data-error-callback",
      "data-expired-callback",
      "data-faq-item-index",
      "data-feature-id",
      "data-flag-game-text",
      "data-flagged-text",
      "data-full-info",
      "data-game-id",
      "data-game-rating",
      "data-height",
      "data-id",
      "data-key",
      "data-like-count",
      "data-metric-tracker-analytics-code-value",
      "data-metric-tracker-max-event-time-value",
      "data-modal-panel",
      "data-modern-ads-network-id-value",
      "data-modern-ads-publisher-id-value",
      "data-modern-ads-refresh-ads-value",
      "data-modern-ads-refresh-interval-value",
      "data-modern-ads-target",
      "data-modern-ads-targeting-params-value",
      "data-modern-ads-user-premium-value",
      "data-modern-video-ads-ad-tag-url-value",
      "data-modern-video-ads-analytics-code-value",
      "data-modern-video-ads-game-groups-value",
      "data-modern-video-ads-game-id-value",
      "data-modern-video-ads-in-top-kred-grace-period-value",
      "data-modern-video-ads-target",
      "data-modern-video-ads-targeting-params-value",
      "data-modern-video-ads-user-premium-value",
      "data-modern-video-ads-user-purchased-kreds-recently-value",
      "data-name",
      "data-ot-ignore",
      "data-policy-link",
      "data-previews--auth-modal-active-tab-value",
      "data-previews--auth-modal-default-tab-value",
      "data-previews--auth-modal-open-value",
      "data-previews--auth-modal-target",
      "data-previews--comments-active-user-value",
      "data-previews--comments-comments-path-value",
      "data-previews--comments-game-id-value",
      "data-previews--comments-target",
      "data-previews--content-modal-open-value",
      "data-previews--content-modal-target",
      "data-previews--dropdown-active-value",
      "data-previews--dropdown-target",
      "data-previews--game-controls-authenticated-users-only-value",
      "data-previews--game-controls-container-strategy-value",
      "data-previews--game-controls-current-rating-value",
      "data-previews--game-controls-game-blocked-value",
      "data-previews--game-controls-game-height-value",
      "data-previews--game-controls-game-id-value",
      "data-previews--game-controls-game-name-value",
      "data-previews--game-controls-game-permalink-value",
      "data-previews--game-controls-game-username-value",
      "data-previews--game-controls-game-width-value",
      "data-previews--game-controls-max-game-height-value",
      "data-previews--game-controls-max-game-width-value",
      "data-previews--game-controls-min-game-height-value",
      "data-previews--game-controls-min-game-width-value",
      "data-previews--game-controls-target",
      "data-previews--game-controls-user-authenticated-value",
      "data-previews--game-controls-user-flagged-value",
      "data-previews--game-pwa-install-game-name-value",
      "data-previews--game-pwa-install-target",
      "data-previews--kreds-purchase-default-checkout-title-value",
      "data-previews--kreds-purchase-default-kreds-title-value",
      "data-previews--kreds-purchase-target",
      "data-previews--login-button-origin-value",
      "data-previews--login-form-target",
      "data-previews--pwa-install-enabled-value",
      "data-previews--pwa-install-target",
      "data-previews--search-bar-target",
      "data-previews--sidebar-hidden-value",
      "data-previews--sidebar-target",
      "data-previews--signup-form-messages-value",
      "data-previews--signup-form-target",
      "data-previews--theme-toggle-target",
      "data-previews-login-form-target",
      "data-purpose-id",
      "data-rating",
      "data-rating-type",
      "data-refreshable",
      "data-sitekey",
      "data-size",
      "data-skip-rating-hover",
      "data-slot-name",
      "data-special-feature-id",
      "data-special-purpose-id",
      "data-tab",
      "data-tab-value",
      "data-title",
      "data-turbo-frame",
      "data-turbo-stream",
      "data-turbo-track",
      "data-vendor-id",
      "data-width",
      "defer",
      "dir",
      "disabled",
      "fetchpriority",
      "fill",
      "fill-rule",
      "focusable",
      "for",
      "frameborder",
      "height",
      "hidden",
      "href",
      "hreflang",
      "http-equiv",
      "id",
      "itemprop",
      "itemscope",
      "itemtype",
      "lang",
      "loading",
      "maxlength",
      "media",
      "method",
      "name",
      "onclick",
      "onkeydown",
      "onmouseout",
      "onmouseover",
      "onsubmit",
      "part",
      "placeholder",
      "playsinline",
      "preload",
      "r",
      "rel",
      "required",
      "role",
      "rows",
      "sandbox",
      "sizes",
      "src",
      "stroke",
      "stroke-linecap",
      "stroke-linejoin",
      "stroke-width",
      "style",
      "tabindex",
      "target",
      "theme",
      "title",
      "translate",
      "type",
      "value",
      "viewbox",
      "webkit-playsinline",
      "width",
      "xmlns"
    ],
    "elements_by_name": {
      "a": 822,
      "aside": 1,
      "b": 4,
      "body": 10,
      "br": 1,
      "button": 194,
      "circle": 1,
      "dd": 2,
      "details": 1,
      "div": 2142,
      "dl": 1,
      "dt": 4,
      "em": 5,
      "fieldset": 6,
      "footer": 1,
      "form": 8,
      "h1": 5,
      "h2": 248,
      "h3": 6,
      "h4": 17,
      "h5": 3,
      "head": 10,
      "header": 1,
      "html": 10,
      "iframe": 11,
      "img": 58,
      "input": 332,
      "k-spinner": 1,
      "label": 323,
      "legend": 6,
      "li": 51,
      "link": 40,
      "main": 1,
      "meta": 30,
      "nav": 2,
      "noscript": 2,
      "ol": 1,
      "option": 183,
      "p": 644,
      "path": 372,
      "rect": 1,
      "script": 57,
      "select": 12,
      "span": 1672,
      "strong": 16,
      "style": 13,
      "summary": 1,
      "svg": 371,
      "textarea": 3,
      "title": 2,
      "turbo-frame": 3,
      "ul": 16,
      "video": 3
    },
    "text_nodes": 2973,
    "total_nodes": 11119,
    "truncated": false
  },
  "dropped_events": 0,
  "frame_origins": [
    "about:blank",
    "https://www.kongregate.com"
  ],
  "network": {
    "request_count": 1,
    "requests": [
      {
        "initiator": "script",
        "method": "POST",
        "mime_type": "text/plain",
        "response_headers": {
          "credentials_present": [],
          "kept": {
            "access-control-allow-credentials": {
              "value_length": 4
            },
            "access-control-allow-origin": {
              "value_length": 26
            },
            "alt-svc": {
              "value_length": 46
            },
            "cache-control": "no-cache, no-store, must-revalidate",
            "content-length": "0",
            "content-security-policy-report-only": "script-src 'none'; form-action 'none'; frame-src 'none'; report-uri https://csp.withgoogle.com/csp/scaffolding/ascnsrsggc:201:0",
            "content-type": "text/plain",
            "cross-origin-opener-policy-report-only": {
              "value_length": 39
            },
            "cross-origin-resource-policy": "cross-origin",
            "date": {
              "value_length": 29
            },
            "expires": {
              "value_length": 29
            },
            "pragma": {
              "value_length": 8
            },
            "report-to": {
              "value_length": 141
            },
            "server": {
              "value_length": 6
            }
          }
        },
        "status": 204,
        "type": "Fetch",
        "url": "https://region1.analytics.google.com/g/collect?v=2&tid=G-27ZBDKBBZJ&gtm=45je6b50h2v898382485z8890041191za20gzb890041191zd890041191xf1&_p=1794223762258&gcd=13l3l3l2l1l1&npa=1&dma_cps=a&dma=1&tcfd=10001&_eu=AAAAACQC&cid=65669306.1794223660&frm=1&ibt=1&ngs=1&pscdl=noapi&rcb=7&sr=800x600&uaa=arm&uab=64&uafvl=Not%253AA-Brand%3B8.0.0.0%7CChromium%3B156.0.8078.81%7CGoogle%2520Chrome%3B156.0.8078.81&uam=&uamb=0&uap=macOS&uapv=27.0.1&uaw=0&ul=fr-fr&_s=3&tag_exp=115938466~115938468~118897920~118897930~120213116~120385422~120412526~120420414~120469145~120469153~121246026~121246034~123785232~123814183&sid=1794223660&sct=1&seg=1&dl=https%3A%2F%2Fwww.kongregate.com%2Fen%2Fgames%2Fpreecep%2Fdesktop-tower-defense%2Fembed&dr=https%3A%2F%2Fwww.kongregate.com%2Fen%2Fgames%2Fpreecep%2Fdesktop-tower-defense&dt=Desktop%20Tower%20Defense%20-%20Play%20on%20Kongregate&en=TTFB&ep.web_vitals_measurement_name=TTFB&ep.web_vitals_measurement_id=v5-1794223762352-5413375113334&epn.web_vitals_measurement_value=780&epn.value=780&tfd=6207"
      }
    ]
  },
  "origin": "https://www.kongregate.com/en/games/preecep/desktop-tower-defense",
  "resources": [
    {
      "mime_type": "image/png",
      "type": "Image",
      "url": "https://cdn4.kongcdn.com/assets/files/0002/8642/robot_Kongbot_US.png"
    },
    {
      "mime_type": "image/png",
      "type": "Image",
      "url": "https://cdn2.kongcdn.com/game_icons/0069/3941/ebf5_kong_thumb.png?i10c=img.resize(width:150,height:120)"
    },
    {
      "mime_type": "application/javascript",
      "type": "Script",
      "url": "https://www.googletagmanager.com/gtag/js?id=AW-11071699779&cx=c&gtm=4e6b41"
    },
    {
      "mime_type": "image/png",
      "type": "Image",
      "url": "https://cdn2.kongcdn.com/game_icons/0070/3505/1000x800_Bitharvest2026.png?i10c=img.resize(width:150,height:120)"
    },
    {
      "mime_type": "",
      "type": "Stylesheet",
      "url": "https://drxfab1qw1oer.cloudfront.net/compiled-assets/swiper/swiper"
    },
    {
      "mime_type": "image/jpeg",
      "type": "Image",
      "url": "https://cdn2.kongcdn.com/game_icons/0045/5359/KongThum500400.jpg?i10c=img.resize(width:150,height:120)"
    },
    {
      "mime_type": "image/png",
      "type": "Image",
      "url": "https://cdn2.kongcdn.com/game_icons/0069/7983/icon_kongregate.png?i10c=img.resize(width:150,height:120)"
    },
    {
      "mime_type": "application/javascript",
      "type": "Script",
      "url": "https://www.googletagmanager.com/gtag/js?id=G-27ZBDKBBZJ&cx=c&gtm=4e6b41"
    },
    {
      "mime_type": "image/png",
      "type": "Image",
      "url": "https://cdn3.kongcdn.com/game_icons/0059/1576/icon250x200.png?i10c=img.resize(width:150,height:120)"
    },
    {
      "mime_type": "image/png",
      "type": "Image",
      "url": "https://cdn1.kongcdn.com/game_icons/0070/2571/image__9_.png?i10c=img.resize(width:150,height:113)"
    },
    {
      "mime_type": "",
      "type": "Stylesheet",
      "url": "https://drxfab1qw1oer.cloudfront.net/compiled-assets/tailwindcss/base"
    },
    {
      "mime_type": "text/javascript",
      "type": "Script",
      "url": "https://s0.2mdn.net/instream/video/client.js"
    },
    {
      "mime_type": "font/woff2",
      "type": "Font",
      "url": "https://fonts.gstatic.com/s/opensans/v40/memvYaGs126MiZpBA-UvWbX2vVnXBbObj2OVTS-mu0SC55I.woff2"
    },
    {
      "mime_type": "image/png",
      "type": "Image",
      "url": "https://cdn1.kongcdn.com/game_icons/0064/6770/logo474x379_250x200ratio_.png?i10c=img.resize(width:150,height:120)"
    },
    {
      "mime_type": "application/javascript",
      "type": "Script",
      "url": "https://cdn.jsdelivr.net/npm/videojs-ima@2.1.0/dist/videojs.ima.min.js"
    },
    {
      "mime_type": "application/javascript",
      "type": "Script",
      "url": "https://drxfab1qw1oer.cloudfront.net/compiled-assets/application-213cec649d1a91269ada173e218e028305c68d0d3866a3b447a4729b22c2030f.js"
    },
    {
      "mime_type": "image/png",
      "type": "Image",
      "url": "https://cdn1.kongcdn.com/assets/avatars/defaults/ant.png?i10c=img.resize(width:28)"
    },
    {
      "mime_type": "image/png",
      "type": "Image",
      "url": "https://cdn1.kongcdn.com/game_icons/0069/1970/zombie-512.png?i10c=img.resize(width:150,height:150)"
    },
    {
      "mime_type": "text/javascript",
      "type": "Script",
      "url": "https://imasdk.googleapis.com/js/sdkloader/ima3.js"
    },
    {
      "mime_type": "image/png",
      "type": "Image",
      "url": "https://cdn3.kongcdn.com/assets/files/0002/9361/15_12_28_space_rocket_Orbit_US.png"
    },
    {
      "mime_type": "",
      "type": "Stylesheet",
      "url": "https://drxfab1qw1oer.cloudfront.net/compiled-assets/components/buttons"
    },
    {
      "mime_type": "text/javascript",
      "type": "Script",
      "url": "https://vjs.zencdn.net/8.23.3/video.min.js"
    },
    {
      "mime_type": "text/css",
      "type": "Stylesheet",
      "url": "https://vjs.zencdn.net/8.23.3/video-js.css"
    },
    {
      "mime_type": "image/png",
      "type": "Image",
      "url": "https://cdn2.kongcdn.com/assets/avatars/defaults/eyeball.png?i10c=img.resize(width:28)"
    },
    {
      "mime_type": "application/javascript",
      "type": "Script",
      "url": "https://cdn.jsdelivr.net/npm/videojs-contrib-ads@7.4.0/dist/videojs.ads.min.js"
    },
    {
      "mime_type": "",
      "type": "Stylesheet",
      "url": "https://drxfab1qw1oer.cloudfront.net/compiled-assets/swiper/modules/keyboard"
    },
    {
      "mime_type": "",
      "type": "Stylesheet",
      "url": "https://drxfab1qw1oer.cloudfront.net/compiled-assets/swiper/modules/mousewheel"
    },
    {
      "mime_type": "image/png",
      "type": "Image",
      "url": "https://cdn2.kongcdn.com/assets/avatars/defaults/yummy.png?i10c=img.resize(width:28)"
    },
    {
      "mime_type": "image/png",
      "type": "Image",
      "url": "https://lh3.googleusercontent.com/GHEhQVpzarCTVLjNQybAPQITBaHd1yFPJgWkGNK8hirA0f2XGlk6rCii1fDrX6z3SO3kvUC6BkA78UyCAf9HSCyqrlVRMCsq-hSGL6ccpOkNsOeljRqm=h60"
    },
    {
      "mime_type": "image/png",
      "type": "Image",
      "url": "https://cdn2.kongcdn.com/game_icons/0051/0118/IconoKongregate.png?i10c=img.resize(width:150,height:120)"
    },
    {
      "mime_type": "image/svg+xml",
      "type": "Image",
      "url": "https://drxfab1qw1oer.cloudfront.net/compiled-assets/logos/kongregate-logo-36fb64eb85f7b359a067eb8e1d989b10633a2e322d8b052c1cde7c4a01745754.svg"
    },
    {
      "mime_type": "image/png",
      "type": "Image",
      "url": "https://cdn4.kongcdn.com/user_avatars/0268/9877/J_Frink.png?i10c=img.resize(width:28)"
    },
    {
      "mime_type": "image/png",
      "type": "Image",
      "url": "https://cdn4.kongcdn.com/user_avatars/0050/3421/Demon_A.png?i10c=img.resize(width:28)"
    },
    {
      "mime_type": "image/png",
      "type": "Image",
      "url": "https://cdn1.kongcdn.com/assets/avatars/defaults/bug.png?i10c=img.resize(width:28)"
    },
    {
      "mime_type": "image/jpeg",
      "type": "Image",
      "url": "https://cdn2.kongcdn.com/user_avatars/0048/5374/Blue_Flaming_Skull.jpg?i10c=img.resize(width:28)"
    },
    {
      "mime_type": "image/png",
      "type": "Image",
      "url": "https://cdn3.kongcdn.com/assets/avatars/defaults/frog.png?i10c=img.resize(width:28)"
    },
    {
      "mime_type": "image/jpeg",
      "type": "Image",
      "url": "https://cdn2.kongcdn.com/game_icons/0025/7656/thumbnail-kong.jpg?i10c=img.resize(width:150,height:120)"
    },
    {
      "mime_type": "text/css",
      "type": "Stylesheet",
      "url": "https://fonts.googleapis.com/css?family=Archivo:400,500|Arimo:400,500|Bitter:400,500|EB+Garamond:400,500|Lato|Libre+Baskervill|Libre+Franklin:400,500|Lora:400,500|Google+Sans:regular,medium:400,500|Material+Icons|Google+Symbols|Merriweather|Montserrat:400,500|Mukta:400,500|Muli:400,500|Nunito:400,500|Open+Sans:400,500,600|Open+Sans+Condensed:400,600|Oswald:500|Playfair+Display:400,500|Poppins:400,500|Raleway:400,500|Roboto:400,500|Roboto+Condensed:400,500|Roboto+Slab:400,500|Slabo+27px|Source+Sans+Pro|Ubuntu:400,500|Volkhov&display=swap"
    },
    {
      "mime_type": "image/png",
      "type": "Image",
      "url": "https://cdn4.kongcdn.com/game_icons/0046/3246/250x200KONG.png?i10c=img.resize(width:150,height:120)"
    },
    {
      "mime_type": "image/gif",
      "type": "Image",
      "url": "https://cdn4.kongcdn.com/game_icons/0069/9792/_______2020_06_08_00_09_18_518.gif?i10c=img.resize(width:150,height:120)"
    },
    {
      "mime_type": "",
      "type": "Stylesheet",
      "url": "https://drxfab1qw1oer.cloudfront.net/compiled-assets/swiper/modules/free-mode"
    },
    {
      "mime_type": "text/css",
      "type": "Stylesheet",
      "url": "https://drxfab1qw1oer.cloudfront.net/compiled-assets/application.tailwind-51c2b678227be1f2567d145dde1986dc74c7c1222a6272d270c184f06779c15b.css"
    },
    {
      "mime_type": "",
      "type": "Stylesheet",
      "url": "https://drxfab1qw1oer.cloudfront.net/compiled-assets/tailwindcss/components"
    },
    {
      "mime_type": "image/jpeg",
      "type": "Image",
      "url": "https://cdn4.
```
