# Messaging Application

Cool little project to get to terms with Rust, React, and solidify Golang knowledge.

## The architecture

```

                                                                                    
                                                                                    
                       ┌───────────────────────────┐                                
                       │                           │                                
                       │        Auth service       │                                
                       │ (authentication validated │                                
                       │         in gateway)       │                                
                       │                           │                                
                       └─────────────▲─────────────┘                                
                                     │                                              
                                     │                                              
                                     │                      ┌───────────────────┐   
                                     │                      │                   │   
                                     ├──────────────────────►  Account service  │   
                                     │                      │                   │   
                                     │                      └───────────────────┘   
                                     │                                              
                                     │                                              
                                     │                                              
  ┌──────────────┐                   │                                              
  │              │            ┌──────┴───────┐             ┌─────────────────────┐  
  │    Client    │            │              │             │                     │  
  │ (Web browser ├────────────►  API Gateway ├─────────────►  Messaging service  │  
  │    or CLI)   │            │              │             │     w/ e2e enc.     │  
  │              │            └──────────────┘             │       (Rust)        │  
  └──────┬───────┘                                         │                     │  
         │                                                 └───────────┬─────────┘  
         │                                                             │            
         │                                                             │            
         │                                                             │            
         │                 ┌───────────────────┐                       │            
         │                 │                   │                       │            
         │                 │    Notification   │                       │            
         └─────────────────► socket connection ◄───────────────────────┘            
                           │     (golang)      │                                    
                           │                   │                                    
                           └───────────────────┘                                    
                                                                                    
                                                                                                                                                

```


# Features

- End-to-end encryption (optional, enabled by default)
- Similar to Telegram, just without the phone numbers and you register via email.
- - As a result, users will have profiles, with names, descriptions, avatars and the rest.

