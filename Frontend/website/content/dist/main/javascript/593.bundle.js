"use strict";(self.webpackChunkSummarize=self.webpackChunkSummarize||[]).push([[593],{593:(e,t,n)=>{n.r(t),n.d(t,{default:()=>c});var i=n(743),o=n(841),r=n(254),s=(0,i.vs)("<form><input type=text placeholder=token required><input type=submit value=Login>");const c=e=>{const[t,n]=(0,o.n5)("");return c=(i=s()).firstChild,i.addEventListener("submit",(function(n){n.preventDefault(),console.log("token: ",t()),(async(e,t)=>{let n=(async e=>{let t=(0,r.Ri)("register_email_token");return null==t&&(t=""),(await fetch("http://127.0.0.1:8000/register/verify",{method:"POST",mode:"cors",headers:{"Content-Type":"application/json",register_email_token:t},body:JSON.stringify({verification_token:e()})})).json()})(e).then((e=>{console.log(e),null!=e.register_response_token&&((0,r.TV)("register_verification_token",e.register_response_token,1800),(0,r.Yj)("register_email_token"),t.detailsMode())}));return n})(t,e).then((e=>console.log("response: ",e)))})),c.$$input=e=>n(e.target.value),i;var i,c};(0,i.z_)(["input"])},254:(e,t,n)=>{function i(e,t,n){let i="";if(n){const e=new Date;e.setTime(e.getTime()+60*n*1e3),i="; expires="+e.toUTCString()}document.cookie=e+"="+t+i+"; path=/; secure; SameSite=Strict"}function o(e){const t=document.cookie.split("; ");for(let n=0;n<t.length;n++){const i=t[n].split("=");if(i[0]===e){const e=i[1];return decodeURIComponent(e)}}return null}function r(e){const t=new Date;t.setTime(t.getTime()-864e8);let n="; expires="+t.toUTCString();document.cookie=e+"="+n+"; path=/; secure; SameSite=Strict"}n.d(t,{Ri:()=>o,TV:()=>i,Yj:()=>r})}}]);