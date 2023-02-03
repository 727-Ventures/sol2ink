"use strict";(self.webpackChunksol_2_ink=self.webpackChunksol_2_ink||[]).push([[974],{3905:(e,t,r)=>{r.d(t,{Zo:()=>p,kt:()=>m});var n=r(7294);function i(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function a(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,n)}return r}function o(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?a(Object(r),!0).forEach((function(t){i(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):a(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function l(e,t){if(null==e)return{};var r,n,i=function(e,t){if(null==e)return{};var r,n,i={},a=Object.keys(e);for(n=0;n<a.length;n++)r=a[n],t.indexOf(r)>=0||(i[r]=e[r]);return i}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(n=0;n<a.length;n++)r=a[n],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(i[r]=e[r])}return i}var s=n.createContext({}),c=function(e){var t=n.useContext(s),r=t;return e&&(r="function"==typeof e?e(t):o(o({},t),e)),r},p=function(e){var t=c(e.components);return n.createElement(s.Provider,{value:t},e.children)},u="mdxType",f={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},d=n.forwardRef((function(e,t){var r=e.components,i=e.mdxType,a=e.originalType,s=e.parentName,p=l(e,["components","mdxType","originalType","parentName"]),u=c(r),d=i,m=u["".concat(s,".").concat(d)]||u[d]||f[d]||a;return r?n.createElement(m,o(o({ref:t},p),{},{components:r})):n.createElement(m,o({ref:t},p))}));function m(e,t){var r=arguments,i=t&&t.mdxType;if("string"==typeof e||i){var a=r.length,o=new Array(a);o[0]=d;var l={};for(var s in t)hasOwnProperty.call(t,s)&&(l[s]=t[s]);l.originalType=e,l[u]="string"==typeof e?e:i,o[1]=l;for(var c=2;c<a;c++)o[c]=r[c];return n.createElement.apply(null,o)}return n.createElement.apply(null,r)}d.displayName="MDXCreateElement"},677:(e,t,r)=>{r.r(t),r.d(t,{assets:()=>s,contentTitle:()=>o,default:()=>f,frontMatter:()=>a,metadata:()=>l,toc:()=>c});var n=r(7462),i=(r(7294),r(3905));const a={sidebar_position:3,title:"Parsing a library"},o=void 0,l={unversionedId:"how_it_works/parsing_library",id:"how_it_works/parsing_library",title:"Parsing a library",description:"When parsing a libraray, Sol2Ink will create a plain Rust file, making all functions public so they can be used in the parsed contract. This file definition may include the following:",source:"@site/docs/how_it_works/parsing_library.md",sourceDirName:"how_it_works",slug:"/how_it_works/parsing_library",permalink:"/how_it_works/parsing_library",draft:!1,editUrl:"https://github.com/727-Ventures/sol2ink/tree/main/docs/docs/how_it_works/parsing_library.md",tags:[],version:"current",sidebarPosition:3,frontMatter:{sidebar_position:3,title:"Parsing a library"},sidebar:"tutorialSidebar",previous:{title:"Parsing an interface",permalink:"/how_it_works/parsing_interface"},next:{title:"Parsing a contract",permalink:"/how_it_works/parsing_contract"}},s={},c=[],p={toc:c},u="wrapper";function f(e){let{components:t,...r}=e;return(0,i.kt)(u,(0,n.Z)({},p,r,{components:t,mdxType:"MDXLayout"}),(0,i.kt)("p",null,"When parsing a libraray, Sol2Ink will create a plain Rust file, making all functions public so they can be used in the parsed contract. This file definition may include the following:"),(0,i.kt)("ul",null,(0,i.kt)("li",{parentName:"ul"},"struct definitions"),(0,i.kt)("li",{parentName:"ul"},"enum definitions"),(0,i.kt)("li",{parentName:"ul"},"function definitions"),(0,i.kt)("li",{parentName:"ul"},"documentation comments"),(0,i.kt)("li",{parentName:"ul"},"state variables (only constants)")),(0,i.kt)("p",null,"After Sol2Ink parses a library, it will move on to the assemble part, where it assembles a Rust file for our library (of course using ink! and OpenBrush where possible) from the parsed structures. The output file will contain the parsed library and include all parsed constant members, and will be saved in ",(0,i.kt)("inlineCode",{parentName:"p"},"generated/src/libs/lib_name.rs"),", where ",(0,i.kt)("inlineCode",{parentName:"p"},"lib_name")," is the name of the parsed library. This library will be also exposed in ",(0,i.kt)("inlineCode",{parentName:"p"},"generated/src/libs/mod.rs")," for the project to use. Note that all functions return ",(0,i.kt)("inlineCode",{parentName:"p"},"Result")," by default."),(0,i.kt)("p",null,"To use our library, we can simply import it in our contract, and use its functions."))}f.isMDXComponent=!0}}]);