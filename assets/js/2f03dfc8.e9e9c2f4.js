"use strict";(self.webpackChunksol_2_ink=self.webpackChunksol_2_ink||[]).push([[971],{3905:(e,t,n)=>{n.d(t,{Zo:()=>c,kt:()=>f});var r=n(7294);function i(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function a(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function s(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?a(Object(n),!0).forEach((function(t){i(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):a(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function o(e,t){if(null==e)return{};var n,r,i=function(e,t){if(null==e)return{};var n,r,i={},a=Object.keys(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||(i[n]=e[n]);return i}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(i[n]=e[n])}return i}var p=r.createContext({}),l=function(e){var t=r.useContext(p),n=t;return e&&(n="function"==typeof e?e(t):s(s({},t),e)),n},c=function(e){var t=l(e.components);return r.createElement(p.Provider,{value:t},e.children)},u="mdxType",d={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},m=r.forwardRef((function(e,t){var n=e.components,i=e.mdxType,a=e.originalType,p=e.parentName,c=o(e,["components","mdxType","originalType","parentName"]),u=l(n),m=i,f=u["".concat(p,".").concat(m)]||u[m]||d[m]||a;return n?r.createElement(f,s(s({ref:t},c),{},{components:n})):r.createElement(f,s({ref:t},c))}));function f(e,t){var n=arguments,i=t&&t.mdxType;if("string"==typeof e||i){var a=n.length,s=new Array(a);s[0]=m;var o={};for(var p in t)hasOwnProperty.call(t,p)&&(o[p]=t[p]);o.originalType=e,o[u]="string"==typeof e?e:i,s[1]=o;for(var l=2;l<a;l++)s[l]=n[l];return r.createElement.apply(null,s)}return r.createElement.apply(null,n)}m.displayName="MDXCreateElement"},3465:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>p,contentTitle:()=>s,default:()=>d,frontMatter:()=>a,metadata:()=>o,toc:()=>l});var r=n(7462),i=(n(7294),n(3905));const a={sidebar_position:6,title:"Parsing expressions"},s=void 0,o={unversionedId:"how_it_works/parsing_expressions",id:"how_it_works/parsing_expressions",title:"Parsing expressions",description:"Another step of parsing a statement is parsing each expression. Here the program will decide how to parse each expression inside a statement.",source:"@site/docs/how_it_works/parsing_expressions.md",sourceDirName:"how_it_works",slug:"/how_it_works/parsing_expressions",permalink:"/how_it_works/parsing_expressions",draft:!1,editUrl:"https://github.com/727-Ventures/sol2ink/tree/main/docs/docs/how_it_works/parsing_expressions.md",tags:[],version:"current",sidebarPosition:6,frontMatter:{sidebar_position:6,title:"Parsing expressions"},sidebar:"tutorialSidebar",previous:{title:"Parsing functions",permalink:"/how_it_works/parsing_functions"},next:{title:"Assembling a contract",permalink:"/how_it_works/assembler"}},p={},l=[{value:"Basics",id:"basics",level:3},{value:"Hex values",id:"hex-values",level:3},{value:"type(T).f / type(T)",id:"typetf--typet",level:3}],c={toc:l},u="wrapper";function d(e){let{components:t,...n}=e;return(0,i.kt)(u,(0,r.Z)({},c,n,{components:t,mdxType:"MDXLayout"}),(0,i.kt)("p",null,"Another step of parsing a statement is parsing each expression. Here the program will decide how to parse each expression inside a statement."),(0,i.kt)("h3",{id:"basics"},"Basics"),(0,i.kt)("ul",null,(0,i.kt)("li",{parentName:"ul"},"Literals are parsed without any modifications"),(0,i.kt)("li",{parentName:"ul"},"Specific expressions like ",(0,i.kt)("inlineCode",{parentName:"li"},"address(0)"),", ",(0,i.kt)("inlineCode",{parentName:"li"},"msg.sender")," or ",(0,i.kt)("inlineCode",{parentName:"li"},"msg.value")," are parsed in their ink! form"),(0,i.kt)("li",{parentName:"ul"},"Solidity types are converted to Rust/ink! types")),(0,i.kt)("h3",{id:"hex-values"},"Hex values"),(0,i.kt)("p",null,"Expressions like ",(0,i.kt)("inlineCode",{parentName:"p"},'hex"0000_0000_0000_0000"')," are converted to a call of ",(0,i.kt)("inlineCode",{parentName:"p"},"&hex::decode")," function."),(0,i.kt)("h3",{id:"typetf--typet"},"type(T).f / type(T)"),(0,i.kt)("p",null,"These expressions are parsed as expected, except ",(0,i.kt)("inlineCode",{parentName:"p"},"type")," is changed to ",(0,i.kt)("inlineCode",{parentName:"p"},"type_of")," since ",(0,i.kt)("inlineCode",{parentName:"p"},"type")," is a keyword in rust. This can produce uncompilable code, since ",(0,i.kt)("inlineCode",{parentName:"p"},"type(uint256).max")," will be parsed as ",(0,i.kt)("inlineCode",{parentName:"p"},"type_of(u128).max")," instead of ",(0,i.kt)("inlineCode",{parentName:"p"},"u128::MAX"),", and the developer needs to change this call. We plan on better support for such functions in the future version of Sol2Ink."),(0,i.kt)("p",null,"All other expressions are parsed as expected:"),(0,i.kt)("ul",null,(0,i.kt)("li",{parentName:"ul"},"struct initializations"),(0,i.kt)("li",{parentName:"ul"},"function calls"),(0,i.kt)("li",{parentName:"ul"},"arithmetic operations"),(0,i.kt)("li",{parentName:"ul"},"logical operations"),(0,i.kt)("li",{parentName:"ul"},"parentheses")),(0,i.kt)("p",null,"After Sol2Ink parses everything, it will assemble the final ink! contract."))}d.isMDXComponent=!0}}]);