# GitHub Label Adder

This simple application adds a label to a GitHub issue or pull request.

This application is most useful when used alongside other tools such as in a container based CI system or using a tool like [Brigade](https://brigade.sh).

## Usage

Prior to running the following environment variables are required to be set:

* `GITHUB_REPO`: The repo (e.g., `mattfarina/github-label-adder`)
* `GITHUB_ISSUE_LABEL`: The label name to add (e.g., `bug`)
* `GITHUB_ISSUE_NUMBER`: The issue number to add it to (e.g., `1`)
* `GITHUB_TOKEN`: The GitHub API token to use for auth

### Brigade Usage Example

Since I am initially using this with brigade here is a quick example from there:

```js
const ghData = JSON.parse(e.payload);
var j = new Job(`just-a-test`, "mattfarina/github-label-adder:latest");
j.env = {
    GITHUB_REPO: project.repo.name,
    GITHUB_ISSUE_LABEL: "invalid",
    GITHUB_TOKEN: project.secrets.ghToken,
    GITHUB_ISSUE_NUMBER: ghData.number.toString(),
}
j.run();
```