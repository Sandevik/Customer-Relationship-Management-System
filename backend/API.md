
# Routes

## /auth
###  /auth/sign-in        | POST | JSON: email (string), password (string)
###  /auth/sign-up        | POST | JSON: email (string), password (string), phone_number (string)
###  /auth/validate-token | POST | JSON: token (string)

##  /crm
###     /crm?crmUuid={insert-uuid-here} | GET,  Secured by ownership 
gets a crm with a specific uuid
###     /crm?crmUuid={insert-uuid-here} | DELETE, Secured by ownership
deletes a crm with a specific uuid
###     /crm/all                     | GET, Secured by ownership




## /all-crms | GET
returns all the logged in users crms
## /create-crm | POST | JSON: name (string)
creates a new crm






## /customers
###     /customers?crmUuid={crm_uuid}&customerUuid={customer_uuid} | GET, Secured by ownership
gets a specific customer with the customer uuid
###     /customers/create | POST | JSON: crmUUid (string), firstName (string), lastName (string), dateOfBirth (string), email (string), address (string), zipCode (string), city (string), country (string), company (string), phoneNumber (string), newsLetter (string)
creates a new customer on a certain crm
##      /customers?crmUuid={crm_uuid} | PUT | JSON: uuid (string), firstName (string), lastName (string), dateOfBirth (string), email (string), address (string), zipCode (string), city (string), country (string), company (string), phoneNumber (string), newsLetter (string)
Updates a specific customer
##      /customers?crmUuid={crm_uuid}&uuid={customer_uuid} | DELETE
removes a specific customer
##      /customers/note?crmUuid={crm_uuid} | PUT | JSON: uuid (string), note (string)
##      /customers/statistics?crmUuid={crm_uuid}&customerUuid={customer_uuid} | GET


## /meetings
###     /meetings/this-month?uuid={crm_uuid} | GET, Secured by ownership
gets all meetings this month
###     /meetings/by-month?crmUuid={crm_uuid}&year={year}&month={month, 1-12} | GET, Secured by ownership, 
returns all meetings based on year and month in query
###     /meetings/create | POST | JSON: crmUuid (string), from (string), to (string), customerUuid (string), Secured by ownership
Creates a new meeting in a crm system with a specific customer
###     /meetings?crmUuid={crm_uuid}&uuid={meeting.uuid} | GET, Secured by ownership
Gets a specific meeting
###     /meetings?crmUuid={crm_uuid}&uuid={meeting.uuid} | DELETE, Secured by ownership
Deletes a specific meeting
###     /meetings/by-customer?crmUuid={crm_uuid}&customerUuid={customer_uuid} | GET, Secured by ownership
Gets all meetings of a certain customer
###     /meetings?crmUuid={crm_uuid}&uuid={meeting_uuid} | PUT | JSON: from (number), to (number), customerUuid (string)
Updates a meeting 




## /entries
###     /entries?crmUuid={crm_uuid}&customerUuid={customer_uuid}&id={entry_id} | PUT | JSON: content (string), addedAtMeeting (string | null)
Updates an entry
###     /entries/create | POST | JSON: crmUuid (string), customerUuid (string), content (string), addedAtMeeting (string | null)
Creates a new entry for a specific customer
###     /entries/all?crmUuid={crm_uuid}&customerUuid={customer_uuid} | GET, Secured by ownership
gets all entries for a specific customer
###     /entries?crmUuid={crm_uuid}&id={entry_id} | DELETE, Secured by ownership
removes a specific entry by its id


## /tasks
###     /tasks/create | POST | JSON: title (string | null), crmUuid (string), deadline (number | null), status (string | null), customerUuid (string | null)
Creates a new Task
###     /tasks/by-customer?crmUuid={crm_uuid}&customerUuid={customer_uuid}
Gets all task revolving a certain customer
###     /tasks/complete | POST | JSON: crmUuid (string), taskUuid (string)
Completes a task regardless if it is reaccuring or not


## /users
### /users/update-language | POST | JSON: language (string), userUuid: (string),
updates the preferred language



## /test
###  /test/generate-hash | GET
















