var searchIndex = JSON.parse('{\
"comet_eventbus":{"doc":"An universal eventbus for Rust!","t":[3,3,6,3,8,3,6,3,23,11,11,11,11,11,11,11,11,11,11,0,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,3,3,3,3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["Event","EventListener","EventListeners","Eventbus","Listener","Topic","TopicHandlersMap","TopicKey","async_trait","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","bridge","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","create_topic","default","deref","deref_mut","downcast","eq","eq","equivalent","equivalent","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","get_bus","get_key","get_listeners","handle","hash","hash","into","into","into","into","into","into_inner","into_request","into_request","into_request","into_request","into_request","ne","new","new","post","post","register","serialized","to_owned","to_owned","to_owned","to_owned","to_string","try_as_str","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","unregister","unregister","vzip","vzip","vzip","vzip","vzip","BridgeListener","BridgedEventListener","BridgedTopic","EventbusBridge","SerializedMessage","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone_into","connect","create_topic","fmt","fmt","fmt","fmt","from","from","from","from","from","get_key","handle","into","into","into","into","into","into_request","into_request","into_request","into_request","into_request","listen","new","post","post","register","to_owned","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","unregister","unregister","vzip","vzip","vzip","vzip","vzip"],"q":["comet_eventbus","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","comet_eventbus::bridge","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["An <code>Event</code> for passing","An <code>EventListener</code> wrapper for <code>Listener</code>","short hand of event listeners set","An asynchronous <code>Eventbus</code> to interact with","Event listener","A <code>Topic</code> wrapper for a <code>TopicKey</code>","short hand of topic to handlers map","Wrapper of bytes represent a <code>Topic</code>","","","","","","","","","","","","bridge <code>Eventbus</code> from an external source","","","","","","","","","create a <code>Topic</code> using a topic key","","","","downcast a Serialized Event to a concreate type.","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","Returns the argument unchanged.","get the associated eventbus","get the key of a topic","get event listeners subscribed to this topic","handler callback to process event","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","into inner message","","","","","","","create an new event","create an new eventbus","shorthand for post event to eventbus","post an event to eventbus","register a listener to eventbus","serialize a message","","","","","","try parse topic key as an utf-8 str","","","","","","","","","","","","","","","","shorthand for unregister listener from eventbus","unregister an event listener","","","","","","Bridge Serialized Event to an concreate typed Event","An <code>EventListener</code> wrapper for <code>Listener</code>","A bridge <code>Topic</code>","A bridge to connect two seperated <code>Eventbus</code>","An serialized message","","","","","","","","","","","","","connect to another Eventbus","create a <code>Topic</code> using a topic key","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","get topic key","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","bind to an address and listen for connections","create a new bridge from an exist <code>Eventbus</code>","shorthand for post event to eventbus","post an event to eventbus","register a listener to bridged eventbus","","","","","","","","","","","","","","","","","unregister a bridged event listener","shorthand for unregister bridged listener from eventbus","","","","",""],"i":[0,0,0,0,0,0,0,0,0,1,2,3,4,5,1,2,3,4,5,0,2,3,4,5,2,3,4,5,4,4,2,2,2,3,5,3,5,1,2,3,4,5,5,1,2,3,4,5,5,1,1,1,6,3,5,1,2,3,4,5,2,1,2,3,4,5,5,2,4,1,4,4,2,2,3,4,5,5,5,1,2,3,4,5,1,2,3,4,5,1,2,3,4,5,3,4,1,2,3,4,5,0,0,0,0,0,7,8,9,10,11,7,8,9,10,11,10,10,10,10,8,9,10,11,7,8,9,10,11,8,7,7,8,9,10,11,7,8,9,10,11,10,10,8,10,10,10,7,8,9,10,11,7,8,9,10,11,7,8,9,10,11,10,11,7,8,9,10,11],"f":[null,null,null,null,null,null,null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],null,[[["",0]]],[[["",0]]],[[["",0]],["eventbus",3]],[[["",0]],["topickey",3]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["into",8,[["topickey",3]]]]],[[]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["option",4,[["event",3,[["",26,[["sized",8],["deserializeowned",8]]]]]]]],[[["",0],["",0]],["bool",0]],[[["",0],["topickey",3]],["bool",0]],[[["",0],["",0]],["bool",0]],[[["",0],["",0]],["bool",0]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[]],[[]],[[["",0]],["eventbus",3]],[[["",0]],["topickey",3]],[[["",0]],["eventlisteners",6]],[[["",0],["event",3]],["pin",3,[["box",3,[["future",8]]]]]],[[["",0],["",0]]],[[["",0],["",0]]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["request",3]],[[],["request",3]],[[],["request",3]],[[],["request",3]],[[],["request",3]],[[["",0],["topickey",3]],["bool",0]],[[["into",8,[["topickey",3]]]]],[[]],[[["",0],["event",3]]],[[["",0],["event",3]]],[[["",0],["into",8,[["topickey",3]]],["listener",8]]],[[["",0]],["option",4,[["event",3,[["serializedmessage",3]]]]]],[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]],["string",3]],[[["",0]],["result",4,[["str",0],["utf8error",3]]]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[]],[[["",0],["eventlistener",3]]],[[]],[[]],[[]],[[]],[[]],null,null,null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["eventbusbridge",3]],[[["",0],["",0]]],[[["",0],["asref",8,[["str",0]]]]],[[["",0],["into",8,[["topickey",3]]]]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[]],[[["",0]],["topickey",3]],[[["",0],["event",3]],["pin",3,[["box",3,[["future",8]]]]]],[[]],[[]],[[]],[[]],[[]],[[],["request",3]],[[],["request",3]],[[],["request",3]],[[],["request",3]],[[],["request",3]],[[["socketaddr",4]]],[[["eventbus",3]]],[[["",0],["event",3]]],[[["",0],["event",3]]],[[["",0],["into",8,[["topickey",3]]],["listener",8,[["",26,[["deserializeowned",8],["send",8],["sync",8]]]]]]],[[["",0]]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0],["bridgedeventlistener",3]]],[[]],[[]],[[]],[[]],[[]],[[]]],"p":[[3,"Topic"],[3,"Event"],[3,"EventListener"],[3,"Eventbus"],[3,"TopicKey"],[8,"Listener"],[3,"BridgeListener"],[3,"BridgedTopic"],[3,"SerializedMessage"],[3,"EventbusBridge"],[3,"BridgedEventListener"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};