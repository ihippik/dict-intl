# Dict-intl

This cli-tool helps us to create a dictionary for the translation library [React-Intl](https://formatjs.io/docs/react-intl/)

After we have replaced all plain text with translation components in our React app, 
we need dictionaries which we will translate into different languages.

### Work logic

* You specify your React project folder and the application recursively scans all files in it.
* We are looking for all the components for translation and collect the default id and message from there: <br> 
```<FormattedMessage id="common.find" defaultMessage="find"/>```
* from this data we collect a dictionary and store it in the json-file <br>
```json
{
  "common.find": "find"
}
```

### Example
#### Configuration

| flag | description                 | default         |
|------|-----------------------------|-----------------|
| -s   | project source directory    | -               |
| -o   | output dictionary file name | dictionary.json |

#### Shell
```shell
./dict-itl -s "./src" -o "dictionary.json"
```