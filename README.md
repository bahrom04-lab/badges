badges - img.shields.io link generator for forgejo.

### Usage

simply add this to `readme.md` file:

```
https://badges.aelloc.uz/?instance=<forgejo instance>&owner=<owner>&repo=<repo>
```

we used to have github badges for one of our repositories in github. but when we
moved to our self-hosted forgejo, those badges were simply misleading and
pointing to our old github repository. i wanted to fix that, but our badge
provides `img.shields.io` didn't support forgejo(as the time of writing this),
so i needed to figure that out. i checked forgejo docs and fount out they have
native badges. we had one for ci and one for most used language, forgejo had one
for ci but it didn't have any for most used language, it was a bit disappointing
but an extremely good opportunity to spend a weekend. then i checked the
`img.shields.io` documentation to learn more about badges, but i stopped at
constructing badges myself. i had an idea how to constuct a badge and needed the
repository data to use it to constuct the link. so i went to check the forgejo
api docs. there was indeed a special endpoint to get all used languages and
bytes of code in those languages as a hashmap. so what i had to do was to fetch
that json and simply put them in the url and redirect to it.

enough yapping, so the repository language information was at
`api/v1/repos/xinux/settings/languages` endpoint. in case of my nixos configs at
codeberg it is:

```bash
curl -X 'GET' \
        'https://codeberg.org/api/v1/repos/aelloc/nihh/languages' \
        -H 'accept: application/json' | jq
```

the result would be:

```json
{
    "Lua": 72903,
    "Nix": 54561,
    "QML": 309
}
```

and let's calculate the percentage using those:

```
72,903(lua, most used language)/ 72,903 + 54,561 + 309(sum😭) = 72,903/127,773
72,903/127,773 = 0.57
0.57 * 100 = 57%
```

moreover, i got the language so the link would be something like this:

```URL
https://img.shields.io/badge/Lua-57%25-blue?logo=Lua
```

and that was it :)

### Deployment

the rust version is ready for deployment but i don't have anything to host it so
that's why i wrote the simpler version in typescript to host on serverless deno
deploy and binded to `badges.aelloc.uz`

i'm hoping to deploy the rust version because it took more effort to write than
the typescript version.
