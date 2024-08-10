# Cleaning 
- `/cleaning` - GET - id of room supposed to clean up the kitchen this week
- `/cleaning/{room-id}` - GET - week (start and end date) when room should clean the kitchen
which room have to clean-up on a specified week
- `/cleaning/{week}` - GET - room that has to clean-up on a specified week
- `/cleaning/{room-id}` - **supervisor only** -  POST, PUT - week on which room has to clean-up
- `/cleaning/{week}` - **supervisor only** - POST, PUT - room that should do the cleaning-up on specified week