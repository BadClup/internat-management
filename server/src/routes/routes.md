# Routes
This is quick overview of our RESTful API.

## Shared

- /login - POST

### Catering
- /rating/catering - GET - list of ratings of all food types 
- /rating/catering/{food-id} - GET - get rating of specific food

### Room
- /rating/room/ - GET - list of ratings of all rooms
- /rating/room/{room-id} - GET - rating of specific room

### Presence
- /presence/{user-id} - GET - get location of the pupil

## Pupils

### Catering
- /rating/catering/{food-id} - POST - add new rating
- /rating/catering/{food-id} - PATCH - update your current rating
- /rating/catering/{food-id} - DELETE - delete your rating

### Presence
- /presence - GET - if you are in the internat or not
- /presence - PUT - if you are going to Europa, use it

## Manager

- /register-many - POST - allows the internat manager to add new pupil

### Presence
- /presence/{user-id} - PUT - change location of pupil

### Room
- /rating/room/{room-id} - POST - rate room
- /rating/room/{room-id} - PUT - update ratring of room