pub mod chart {
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Viewport {
        #[prost(double, tag = "1")]
        pub width: f64,
        #[prost(double, tag = "2")]
        pub height: f64,
        #[prost(message, optional, tag = "3")]
        pub margin: ::core::option::Option<Margin>,
        #[prost(message, optional, tag = "4")]
        pub domain: ::core::option::Option<Extent>,
        #[prost(message, optional, tag = "5")]
        pub range: ::core::option::Option<Extent>,
    }

    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Margin {
        #[prost(double, tag = "1")]
        pub top: f64,
        #[prost(double, tag = "2")]
        pub right: f64,
        #[prost(double, tag = "3")]
        pub bottom: f64,
        #[prost(double, tag = "4")]
        pub left: f64,
    }

    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Extent {
        #[prost(double, tag = "1")]
        pub minimum: f64,
        #[prost(double, tag = "2")]
        pub maximum: f64,
    }

    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Term {
        #[prost(uint32, tag = "1")]
        pub exponent: u32,
        #[prost(double, tag = "2")]
        pub coefficient: f64,
    }

    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Sample {
        #[prost(double, tag = "1")]
        pub point: f64,
        #[prost(double, tag = "2")]
        pub mean: f64,
        #[prost(double, tag = "3")]
        pub deviation: f64,
        #[prost(double, tag = "4")]
        pub predicted: f64,
        #[prost(double, tag = "5")]
        pub lower: f64,
        #[prost(double, tag = "6")]
        pub upper: f64,
        #[prost(uint32, tag = "7")]
        pub count: u32,
    }

    #[derive(Clone, PartialEq, prost::Message)]
    pub struct Chart {
        #[prost(message, optional, tag = "1")]
        pub viewport: ::core::option::Option<Viewport>,
        #[prost(double, tag = "2")]
        pub normalization: f64,
        #[prost(string, tag = "3")]
        pub label: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "4")]
        pub terms: ::prost::alloc::vec::Vec<Term>,
        #[prost(message, repeated, tag = "5")]
        pub samples: ::prost::alloc::vec::Vec<Sample>,
    }
}
