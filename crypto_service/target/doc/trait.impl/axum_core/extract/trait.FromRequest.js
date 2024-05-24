(function() {var implementors = {
"axum":[["impl&lt;S, B&gt; <a class=\"trait\" href=\"axum/extract/trait.FromRequest.html\" title=\"trait axum::extract::FromRequest\">FromRequest</a>&lt;S, B&gt; for <a class=\"struct\" href=\"axum/extract/struct.BodyStream.html\" title=\"struct axum::extract::BodyStream\">BodyStream</a><div class=\"where\">where\n    B: <a class=\"trait\" href=\"http_body/trait.Body.html\" title=\"trait http_body::Body\">HttpBody</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'static,\n    B::<a class=\"associatedtype\" href=\"http_body/trait.Body.html#associatedtype.Data\" title=\"type http_body::Body::Data\">Data</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"bytes/bytes/struct.Bytes.html\" title=\"struct bytes::bytes::Bytes\">Bytes</a>&gt;,\n    B::<a class=\"associatedtype\" href=\"http_body/trait.Body.html#associatedtype.Error\" title=\"type http_body::Body::Error\">Error</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"type\" href=\"axum/type.BoxError.html\" title=\"type axum::BoxError\">BoxError</a>&gt;,\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,</div>"],["impl&lt;S, B&gt; <a class=\"trait\" href=\"axum/extract/trait.FromRequest.html\" title=\"trait axum::extract::FromRequest\">FromRequest</a>&lt;S, B&gt; for <a class=\"struct\" href=\"axum/extract/struct.RawBody.html\" title=\"struct axum::extract::RawBody\">RawBody</a>&lt;B&gt;<div class=\"where\">where\n    B: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,</div>"],["impl&lt;S, B&gt; <a class=\"trait\" href=\"axum/extract/trait.FromRequest.html\" title=\"trait axum::extract::FromRequest\">FromRequest</a>&lt;S, B&gt; for <a class=\"struct\" href=\"axum/extract/struct.RawForm.html\" title=\"struct axum::extract::RawForm\">RawForm</a><div class=\"where\">where\n    B: <a class=\"trait\" href=\"http_body/trait.Body.html\" title=\"trait http_body::Body\">HttpBody</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'static,\n    B::<a class=\"associatedtype\" href=\"http_body/trait.Body.html#associatedtype.Data\" title=\"type http_body::Body::Data\">Data</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,\n    B::<a class=\"associatedtype\" href=\"http_body/trait.Body.html#associatedtype.Error\" title=\"type http_body::Body::Error\">Error</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"type\" href=\"axum/type.BoxError.html\" title=\"type axum::BoxError\">BoxError</a>&gt;,\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,</div>"]],
"axum_core":[]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()