# RustGitManager

Found git repos and it's respective state

## Installation

### From Github releases page

Go to [Release page](https://github.com/danny270793/git-manager/releases) then download the binary which fits your environment

### From terminal

Get the last versión available on github

```bash
LAST_VERSION=$(curl https://api.github.com/repos/danny270793/git-manager/releases/latest | grep tag_name | cut -d '"' -f 4)
```

Download the last version directly to the binaries folder

For Linux (linux):

```bash
curl -L https://github.com/danny270793/git-manager/releases/download/${LAST_VERSION}/git-manager -o ./git-manager
```

Then copy the binary to the binaries folder

```bash
sudo cp ./git-manager /usr/local/bin/git-manager
```

Make it executable the binary

```bash
sudo chmod +x /usr/local/bin/git-manager
```

```bash
git-manager version
```

## Ussage

Run the binary and pass the path to the folder where you want to look for missing variables

```bash
git-manager ./path/to/git/repos
```

## Follow me

- [Youtube](https://www.youtube.com/channel/UC5MAQWU2s2VESTXaUo-ysgg)
- [Github](https://www.github.com/danny270793/)
- [LinkedIn](https://www.linkedin.com/in/danny270793)

## LICENSE

Licensed under the [MIT](license.md) License

## Version

RustGitManager version 1.0.0

Last update 07/11/2023
