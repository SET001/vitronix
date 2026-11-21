ids in params or in separated?
```
pub containers: HashMap<String, ContainerParams>,
```
currently ID is separated from container params itself and lies in hashmap key, should we move it in?