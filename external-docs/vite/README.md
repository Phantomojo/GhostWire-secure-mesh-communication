Title: Getting Started | Vite

Description: Next Generation Frontend Tooling

Source: https://vitejs.dev/guide/

---

[Announcing Vite+ Alpha: Open source. Unified. Next-gen.](https://voidzero.dev/posts/announcing-vite-plus-alpha?utm_source=vite&utm_content=top_banner)
[Skip to content](https://vitejs.dev/guide/#VPContent)
[Guide](https://vitejs.dev/guide/)
[Config](https://vitejs.dev/config/)
[Plugins](https://vitejs.dev/plugins/)
[Team](https://vitejs.dev/team)
[Blog](https://vitejs.dev/blog)
[Releases](https://vitejs.dev/releases)
[Acknowledgements](https://vitejs.dev/acknowledgements)
[Plugin Registry](https://registry.vite.dev/plugins)
[The Documentary](https://www.youtube.com/watch?v=bmWQqAKLgT4)
[Bluesky](https://bsky.app/profile/vite.dev)
[Mastodon](https://elk.zone/m.webtoo.ls/@vite)
[X](https://x.com/vite_js)
[Discord Chat](https://chat.vite.dev)
[Awesome Vite](https://github.com/vitejs/awesome-vite)
[ViteConf](https://viteconf.org)
[DEV Community](https://dev.to/t/vite)
[Changelog](https://github.com/vitejs/vite/blob/main/packages/vite/CHANGELOG.md)
[Contributing](https://github.com/vitejs/vite/blob/main/CONTRIBUTING.md)
[Unreleased Docs](https://main.vite.dev)
[Vite 7 Docs](https://v7.vite.dev)
[Vite 6 Docs](https://v6.vite.dev)
[Vite 5 Docs](https://v5.vite.dev)
[Vite 4 Docs](https://v4.vite.dev)
[Vite 3 Docs](https://v3.vite.dev)
[Vite 2 Docs](https://v2.vite.dev)
English
[简体中文](https://cn.vite.dev/guide/)
[日本語](https://ja.vite.dev/guide/)
[Español](https://es.vite.dev/guide/)
[Português](https://pt.vite.dev/guide/)
[한국어](https://ko.vite.dev/guide/)
[Deutsch](https://de.vite.dev/guide/)
[فارسی](https://fa.vite.dev/guide/)
Appearance
English
[简体中文](https://cn.vite.dev/guide/)
[日本語](https://ja.vite.dev/guide/)
[Español](https://es.vite.dev/guide/)
[Português](https://pt.vite.dev/guide/)
[한국어](https://ko.vite.dev/guide/)
[Deutsch](https://de.vite.dev/guide/)
[فارسی](https://fa.vite.dev/guide/)

## Introduction
[Getting Started](https://vitejs.dev/guide/)
Getting Started
[Philosophy](https://vitejs.dev/guide/philosophy)
Philosophy
[Why Vite](https://vitejs.dev/guide/why)
Why Vite

## Guide
[Features](https://vitejs.dev/guide/features)
Features
[CLI](https://vitejs.dev/guide/cli)
CLI
[Using Plugins](https://vitejs.dev/guide/using-plugins)
Using Plugins
[Dependency Pre-Bundling](https://vitejs.dev/guide/dep-pre-bundling)
Dependency Pre-Bundling
[Static Asset Handling](https://vitejs.dev/guide/assets)
Static Asset Handling
[Building for Production](https://vitejs.dev/guide/build)
Building for Production
[Deploying a Static Site](https://vitejs.dev/guide/static-deploy)
Deploying a Static Site
[Env Variables and Modes](https://vitejs.dev/guide/env-and-mode)
Env Variables and Modes
[Server-Side Rendering (SSR)](https://vitejs.dev/guide/ssr)
Server-Side Rendering (SSR)
[Backend Integration](https://vitejs.dev/guide/backend-integration)
Backend Integration
[Troubleshooting](https://vitejs.dev/guide/troubleshooting)
Troubleshooting
[Performance](https://vitejs.dev/guide/performance)
Performance
[Migration from v7](https://vitejs.dev/guide/migration)
Migration from v7
[Breaking Changes](https://vitejs.dev/changes/)
Breaking Changes

## APIs
[Plugin API](https://vitejs.dev/guide/api-plugin)
Plugin API
[HMR API](https://vitejs.dev/guide/api-hmr)
HMR API
[JavaScript API](https://vitejs.dev/guide/api-javascript)
JavaScript API
[Config Reference](https://vitejs.dev/config/)
Config Reference

## Environment API
[Introduction](https://vitejs.dev/guide/api-environment)
Introduction
[Environment Instances](https://vitejs.dev/guide/api-environment-instances)
Environment Instances
[Plugins](https://vitejs.dev/guide/api-environment-plugins)
Plugins
[Frameworks](https://vitejs.dev/guide/api-environment-frameworks)
Frameworks
[Runtimes](https://vitejs.dev/guide/api-environment-runtimes)
Runtimes
[Building TogetherViteConf 2025View the replays](https://www.youtube.com/playlist?list=PLqGQbXn_GDmkJaoykvHCUmXUPjhgH2bVr)
Building Together
ViteConf 2025
View the replays

[​](https://vitejs.dev/guide/#getting-started)

## Overview [​](https://vitejs.dev/guide/#overview)
[​](https://vitejs.dev/guide/#overview)
Vite (French word for "quick", pronounced /viːt/, like "veet") is a build tool that aims to provide a faster and leaner development experience for modern web projects. It consists of two major parts:

```
/viːt/
```

- A dev server that provides [rich feature enhancements](https://vitejs.dev/guide/features) over [native ES modules](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Modules), for example extremely fast [Hot Module Replacement (HMR)](https://vitejs.dev/guide/features#hot-module-replacement).
- A build command that bundles your code with [Rolldown](https://rolldown.rs), pre-configured to output highly optimized static assets for production.
A dev server that provides [rich feature enhancements](https://vitejs.dev/guide/features) over [native ES modules](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Modules), for example extremely fast [Hot Module Replacement (HMR)](https://vitejs.dev/guide/features#hot-module-replacement).
A build command that bundles your code with [Rolldown](https://rolldown.rs), pre-configured to output highly optimized static assets for production.
Vite is opinionated and comes with sensible defaults out of the box. Read about what's possible in the [Features Guide](https://vitejs.dev/guide/features). Support for frameworks or integration with other tools is possible through [Plugins](https://vitejs.dev/guide/using-plugins). The [Config Section](https://vitejs.dev/config/) explains how to adapt Vite to your project if needed.
Vite is also highly extensible via its [Plugin API](https://vitejs.dev/guide/api-plugin) and [JavaScript API](https://vitejs.dev/guide/api-javascript) with full typing support.
You can learn more about the rationale behind the project in the [Why Vite](https://vitejs.dev/guide/why) section.
[Learn Vite through interactive tutorials on Scrimba](https://scrimba.com/intro-to-vite-c03p6pbbdq?via=vite)

## Browser Support [​](https://vitejs.dev/guide/#browser-support)
[​](https://vitejs.dev/guide/#browser-support)
During development, Vite assumes that a modern browser is used. This means the browser supports most of the latest JavaScript and CSS features. For that reason, Vite sets [esnext as the transform target](https://oxc.rs/docs/guide/usage/transformer/lowering.html#target). This prevents syntax lowering, letting Vite serve modules as close as possible to the original source code. Vite injects some runtime code to make the development server work. This code uses features included in [Baseline](https://web-platform-dx.github.io/web-features/) Newly Available at the time of each major release (2026-01-01 for this major).

```
esnext
```

For production builds, Vite by default targets [Baseline](https://web-platform-dx.github.io/web-features/) Widely Available browsers. These are browsers that were released at least 2.5 years ago. The target can be lowered via configuration. Additionally, legacy browsers can be supported via the official [@vitejs/plugin-legacy](https://github.com/vitejs/vite/tree/main/packages/plugin-legacy). See the [Building for Production](https://vitejs.dev/guide/build) section for more details.

## Trying Vite Online [​](https://vitejs.dev/guide/#trying-vite-online)
[​](https://vitejs.dev/guide/#trying-vite-online)
You can try Vite online on [StackBlitz](https://vite.new/). It runs the Vite-based build setup directly in the browser, so it is almost identical to the local setup but doesn't require installing anything on your machine. You can navigate to vite.new/{template} to select which framework to use.

```
vite.new/{template}
```

The supported template presets are:
[vanilla](https://vite.new/vanilla)
[vanilla-ts](https://vite.new/vanilla-ts)
[vue](https://vite.new/vue)
[vue-ts](https://vite.new/vue-ts)
[react](https://vite.new/react)
[react-ts](https://vite.new/react-ts)
[preact](https://vite.new/preact)
[preact-ts](https://vite.new/preact-ts)
[lit](https://vite.new/lit)
[lit-ts](https://vite.new/lit-ts)
[svelte](https://vite.new/svelte)
[svelte-ts](https://vite.new/svelte-ts)
[solid](https://vite.new/solid)
[solid-ts](https://vite.new/solid-ts)
[qwik](https://vite.new/qwik)
[qwik-ts](https://vite.new/qwik-ts)

## Scaffolding Your First Vite Project [​](https://vitejs.dev/guide/#scaffolding-your-first-vite-project)
[​](https://vitejs.dev/guide/#scaffolding-your-first-vite-project)

```
$ npm create vite@latest
```


```
$ npm create vite@latest
```


```
$ yarn create vite
```


```
$ yarn create vite
```


```
$ pnpm create vite
```


```
$ pnpm create vite
```


```
$ bun create vite
```


```
$ bun create vite
```


```
$ deno init --npm vite
```


```
$ deno init --npm vite
```

Then follow the prompts!
[Watch an interactive lesson on Scrimba](https://scrimba.com/intro-to-vite-c03p6pbbdq/~0yhj?via=vite)
Compatibility Note
Vite requires [Node.js](https://nodejs.org/en/) version 20.19+, 22.12+. However, some templates require a higher Node.js version to work, please upgrade if your package manager warns about it.
You can also directly specify the project name and the template you want to use via additional command line options. For example, to scaffold a Vite + Vue project, run:

```
# npm 7+, extra double-dash is needed: $ npm create vite@latest my-vue-app -- --template vue
```


```
# npm 7+, extra double-dash is needed: $ npm create vite@latest my-vue-app -- --template vue
```


```
$ yarn create vite my-vue-app --template vue
```


```
$ yarn create vite my-vue-app --template vue
```


```
$ pnpm create vite my-vue-app --template vue
```


```
$ pnpm create vite my-vue-app --template vue
```


```
$ bun create vite my-vue-app --template vue
```


```
$ bun create vite my-vue-app --template vue
```


```
$ deno init --npm vite my-vue-app --template vue
```


```
$ deno init --npm vite my-vue-app --template vue
```

See [create-vite](https://github.com/vitejs/vite/tree/main/packages/create-vite) for more details on each supported template: vanilla, vanilla-ts, vue, vue-ts, react, react-ts, preact, preact-ts, lit, lit-ts, svelte, svelte-ts, solid, solid-ts, qwik, qwik-ts.

```
vanilla
```


```
vanilla-ts
```


```
vue
```


```
vue-ts
```


```
react
```


```
react-ts
```


```
preact
```


```
preact-ts
```


```
lit
```


```
lit-ts
```


```
svelte
```


```
svelte-ts
```


```
solid
```


```
solid-ts
```


```
qwik
```


```
qwik-ts
```

You can use . for the project name to scaffold in the current directory.

```
.
```

To create a project without interactive prompts, you can use the --no-interactive flag.

```
--no-interactive
```

## Community Templates [​](https://vitejs.dev/guide/#community-templates)
[​](https://vitejs.dev/guide/#community-templates)
create-vite is a tool to quickly start a project from a basic template for popular frameworks. Check out Awesome Vite for [community maintained templates](https://github.com/vitejs/awesome-vite#templates) that include other tools or target different frameworks.
For a template at https://github.com/user/project, you can try it out online using https://github.stackblitz.com/user/project (adding .stackblitz after github to the URL of the project).

```
https://github.com/user/project
```


```
https://github.stackblitz.com/user/project
```


```
.stackblitz
```


```
github
```

You can also use a tool like [tiged](https://github.com/tiged/tiged) to scaffold your project with one of the templates. Assuming the project is on GitHub and uses main as the default branch, you can create a local copy using:

```
main
```


```
npx tiged user/project my-project cd my-project npm install npm run dev
```


```
npx tiged user/project my-project cd my-project npm install npm run dev
```

## Manual Installation [​](https://vitejs.dev/guide/#manual-installation)
[​](https://vitejs.dev/guide/#manual-installation)
In your project, you can install the vite CLI using:

```
vite
```


```
$ npm install -D vite
```


```
$ npm install -D vite
```


```
$ yarn add -D vite
```


```
$ yarn add -D vite
```


```
$ pnpm add -D vite
```


```
$ pnpm add -D vite
```


```
$ bun add -D vite
```


```
$ bun add -D vite
```


```
$ deno add -D npm:vite
```


```
$ deno add -D npm:vite
```

And create an index.html file like this:

```
index.html
```


```
<p>Hello Vite!</p>
```


```
<p>Hello Vite!</p>
```

Then run the appropriate CLI command in your terminal:

```
$ npx vite
```


```
$ npx vite
```


```
$ yarn vite
```


```
$ yarn vite
```


```
$ pnpm vite
```


```
$ pnpm vite
```


```
$ bunx vite
```


```
$ bunx vite
```


```
$ deno run -A npm:vite
```


```
$ deno run -A npm:vite
```

The index.html will be served on http://localhost:5173.

```
index.html
```


```
http://localhost:5173
```

## index.html and Project Root [​](https://vitejs.dev/guide/#index-html-and-project-root)
```
index.html
```

[​](https://vitejs.dev/guide/#index-html-and-project-root)
One thing you may have noticed is that in a Vite project, index.html is front-and-central instead of being tucked away inside public. This is intentional: during development Vite is a server, and index.html is the entry point to your application.

```
index.html
```


```
public
```


```
index.html
```

Vite treats index.html as source code and part of the module graph. It resolves <script type="module" src="..."> that references your JavaScript source code. Even inline <script type="module"> and CSS referenced via <link href> also enjoy Vite-specific features. In addition, URLs inside index.html are automatically rebased so there's no need for special %PUBLIC_URL% placeholders.

```
index.html
```


```
<script type="module" src="...">
```


```
<script type="module">
```


```
<link href>
```


```
index.html
```


```
%PUBLIC_URL%
```

Similar to static http servers, Vite has the concept of a "root directory" which your files are served from. You will see it referenced as <root> throughout the rest of the docs. Absolute URLs in your source code will be resolved using the project root as base, so you can write code as if you are working with a normal static file server (except way more powerful!). Vite is also capable of handling dependencies that resolve to out-of-root file system locations, which makes it usable even in a monorepo-based setup.

```
<root>
```

Vite also supports [multi-page apps](https://vitejs.dev/guide/build#multi-page-app) with multiple .html entry points.

```
.html
```


#### Specifying Alternative Root [​](https://vitejs.dev/guide/#specifying-alternative-root)
[​](https://vitejs.dev/guide/#specifying-alternative-root)
Running vite starts the dev server using the current working directory as root. You can specify an alternative root with vite serve some/sub/dir. Note that Vite will also resolve [its config file (i.e. vite.config.js)](https://vitejs.dev/config/#configuring-vite) inside the project root, so you'll need to move it if the root is changed.

```
vite
```


```
vite serve some/sub/dir
```


```
vite.config.js
```

## Command Line Interface [​](https://vitejs.dev/guide/#command-line-interface)
[​](https://vitejs.dev/guide/#command-line-interface)
In a project where Vite is installed, you can use the vite binary in your npm scripts, or run it directly with npx vite. Here are the default npm scripts in a scaffolded Vite project:

```
vite
```


```
npx vite
```


```
{ "scripts": { "dev": "vite", // start dev server, aliases: `vite dev`, `vite serve` "build": "vite build", // build for production "preview": "vite preview" // locally preview production build } }
```


```
{ "scripts": { "dev": "vite", // start dev server, aliases: `vite dev`, `vite serve` "build": "vite build", // build for production "preview": "vite preview" // locally preview production build } }
```

You can specify additional CLI options like --port or --open. For a full list of CLI options, run npx vite --help in your project.

```
--port
```


```
--open
```


```
npx vite --help
```

Learn more about the [Command Line Interface](https://vitejs.dev/guide/cli)

## Using Unreleased Commits [​](https://vitejs.dev/guide/#using-unreleased-commits)
[​](https://vitejs.dev/guide/#using-unreleased-commits)
If you can't wait for a new release to test the latest features, you can install a specific commit of Vite with [https://pkg.pr.new](https://pkg.pr.new):

```
$ npm install -D https://pkg.pr.new/vite@SHA
```


```
$ npm install -D https://pkg.pr.new/vite@SHA
```


```
$ yarn add -D https://pkg.pr.new/vite@SHA
```


```
$ yarn add -D https://pkg.pr.new/vite@SHA
```


```
$ pnpm add -D https://pkg.pr.new/vite@SHA
```


```
$ pnpm add -D https://pkg.pr.new/vite@SHA
```


```
$ bun add -D https://pkg.pr.new/vite@SHA
```


```
$ bun add -D https://pkg.pr.new/vite@SHA
```

Replace SHA with any of [Vite's commit SHAs](https://github.com/vitejs/vite/commits/main/). Note that only commits within the last month will work, as older commit releases are purged.

```
SHA
```

Alternatively, you can also clone the [vite repo](https://github.com/vitejs/vite) to your local machine and then build and link it yourself ([pnpm](https://pnpm.io/) is required):

```
git clone https://github.com/vitejs/vite.git cd vite pnpm install cd packages/vite pnpm run build pnpm link # use your preferred package manager for this step
```


```
git clone https://github.com/vitejs/vite.git cd vite pnpm install cd packages/vite pnpm run build pnpm link # use your preferred package manager for this step
```

Then go to your Vite based project and run pnpm link vite (or the package manager that you used to link vite globally). Now restart the development server to ride on the bleeding edge!

```
pnpm link vite
```


```
vite
```

To learn more about how and when Vite does releases, check out the [Releases](https://vitejs.dev/releases) documentation.
Dependencies using Vite
To replace the Vite version used by dependencies transitively, you should use [npm overrides](https://docs.npmjs.com/cli/v11/configuring-npm/package-json#overrides) or [pnpm overrides](https://pnpm.io/9.x/package_json#pnpmoverrides).

## Community [​](https://vitejs.dev/guide/#community)
[​](https://vitejs.dev/guide/#community)
If you have questions or need help, reach out to the community at [Discord](https://chat.vite.dev) and [GitHub Discussions](https://github.com/vitejs/vite/discussions).
[Suggest changes to this page](https://github.com/vitejs/vite/edit/main/docs/guide/index.md)
[Next pagePhilosophy](https://vitejs.dev/guide/philosophy)
© 2019-present VoidZero Inc. and Vite contributors. (640202a2)

