crate::define_builder! {
    #[derive(Debug)]
    builder VariantBuilder {
        //
        // Setup frame
        //
        #[derive(Debug)]
        SetupBuilder for super::Setup: Setup => {
            // terminals
            .build;

            // flags
            pub lease => sets [LEASE];

            // fields
            pub version: super::Version;
            pub keepalive(required): super::NonZero<u32>;
            pub lifetime(required): super::NonZero<u32>;
            pub token: super::ResumeToken => sets [RESUME];
            pub mime_metadata(required): super::MimeType;
            pub mime_data(required): super::MimeType;
            pub metadata: Option<super::Metadata> => sets [METADATA if metadata.is_some()];
            pub data: super::Data;
        };

        //
        // Error frame
        //
        #[derive(Debug)]
        ErrorBuilder for super::Error: Error => {
            // terminals
            .build;

            // fields
            pub code(required): super::ErrorCode;
            pub data(required): recode::codec::Utf8;
        };

        //
        // Lease frame
        //
        #[derive(Debug)]
        LeaseBuilder for super::Lease: Lease => {
            // terminals
            .build;

            // fields
            pub ttl(required): super::NonZero::<u32>;
            pub number_of_requests(required): super::NonZero::<u32>;
            pub metadata: Option<super::Metadata> => sets [METADATA if metadata.is_some()];
        };

        //
        // Keepalive frame
        //
        #[derive(Debug)]
        KeepaliveBuilder for super::Keepalive: Keepalive => {
            // terminals
            .build;

            // flags
            pub respond => sets [ RESPOND ];

            // fields
            pub last_received_position(required): u64;
            pub data: super::Data;
        };

        //
        // Request/Response frame
        //
        #[derive(Debug)]
        RequestResponseBuilder for super::RequestResponse: RequestResponse => {
            // terminals
            .build;
            .incomplete => sets [FOLLOW];

            // fields
            pub metadata: Option<super::Metadata> => sets [METADATA if metadata.is_some()];
            pub data: super::Data;
        };

        //
        // Request/FNF frame
        //
        #[derive(Debug)]
        RequestFNFBuilder for super::RequestFNF: RequestFNF => {
            // terminals
            .build;
            .incomplete => sets [FOLLOW];

            // fields
            pub metadata: Option<super::Metadata> => sets [METADATA if metadata.is_some()];
            pub data: super::Data;
        };

        //
        // Request/Stream frame
        //
        #[derive(Debug)]
        RequestStreamBuilder for super::RequestStream: RequestStream => {
            // terminals
            .build;
            .incomplete => sets [FOLLOW];

            // fields
            pub initial_request_n(required): super::NonZero<u32>;
            pub metadata: Option<super::Metadata> => sets [METADATA if metadata.is_some()];
            pub data: super::Data;
        };

        //
        // Request/Channel frame
        //
        #[derive(Debug)]
        RequestChannelBuilder for super::RequestChannel: RequestChannel => {
            // terminals
            .build;
            .incomplete => sets [FOLLOW];

            // flags
            pub complete => sets [COMPLETE];

            // fields
            pub initial_request_n(required): super::NonZero<u32>;
            pub metadata: Option<super::Metadata> => sets [METADATA if metadata.is_some()];
            pub data: super::Data;
        };

        //
        // Request N frame
        //
        #[derive(Debug)]
        RequestNBuilder for super::RequestN: RequestN => {
            // terminals
            .build;

            // fields
            pub request_n(required): super::NonZero<u32>;
        };

        //
        // Cancel frame
        //
        #[derive(Debug)]
        CancelBuilder for super::Cancel: Cancel => {
            // terminals
            .build;
        };

        //
        // Payload frame
        //
        #[derive(Debug)]
        PayloadBuilder for super::Payload: Payload => {
            // terminals
            .build;
            .incomplete => sets [FOLLOW];

            // flags
            pub complete => sets [COMPLETE];
            pub next => sets [NEXT];

            // fields
            pub metadata: Option<super::Metadata> => sets [
                METADATA if metadata.is_some(),
                NEXT if metadata.is_some()
            ];
            pub data: Option<super::Data> => sets [ NEXT if data.is_some() ];
        };

        //
        // Metadata Push frame
        //
        #[derive(Debug)]
        MetadataPushBuilder for super::MetadataPush: MetadataPush => {
            // terminals
            .build;

            // fields
            pub metadata(required): super::Metadata => sets [METADATA];
        };

        //
        // Resume frame
        //
        #[derive(Debug)]
        ResumeBuilder for super::Resume: Resume => {
            // terminals
            .build;

            // fields
            pub version(required): super::Version;
            pub resume_identification_token(required): super::ResumeToken;
            pub last_received_server_position(required): u64;
            pub first_available_client_position(required): u64;
        };

        //
        // ResumeOk frame
        //
        #[derive(Debug)]
        ResumeOkBuilder for super::ResumeOk: ResumeOk => {
            // terminals
            .build;

            // fields
            pub last_received_client_position(required): u64;
        };
    }
}
