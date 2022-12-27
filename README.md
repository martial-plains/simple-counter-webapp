## [Yew](https://yew.rs/) Template for Github.io

Yew template that deployable as is for github.io with [tailwind.css](https://tailwindcss.com/) and webpack with your css/scss and [trunk](https://trunkrs.dev) for build.

[About github.io](https://pages.github.com/)

## For {username}.github.io page

you need to modify [this line](https://github.com/a-isaiahharvey/yew-template-for-github-io/blob/main/Trunk.toml#L5-L7)
and [this line](https://github.com/a-isaiahharvey/yew-template-for-github-io/blob/main/static/404.html#L25) to 0.

## For {username}.github.io/{project_name} page

you need to modify [this line](https://github.com/a-isaiahharvey/yew-template-for-github-io/blob/main/Trunk.toml#L5-L7)
and [this line](https://github.com/a-isaiahharvey/yew-template-for-github-io/blob/main/static/404.html#L25) to 1.

To serve the project on your machine correctly, run `trunk serve --public-url=/{project_name}/`

## Using Custom Domain

go to `./.github/workflows/publish_gh_pages.yml` and add your domain in cname field.

## Routing

Using `static/404.html`. for more information, checkout https://github.com/rafgraph/spa-github-pages.
