use error_stack::{AttachmentKind, Frame, FrameKind, Report};
use yams_core::ThreadSafeError;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct StructuredError {
    pub message: String,
    pub attachments: Vec<String>,
    pub sources: Vec<StructuredError>,
}

fn into_structured_error_from_frame(frame: &Frame, head: &[&Frame]) -> Vec<StructuredError> {
    let mut attachments = head.to_vec();

    match frame.kind() {
        FrameKind::Context(context) => {
            // Found the context, collect all attachment messages (in reverse order).
            attachments.reverse();
            let attachments_msgs = attachments
                .iter()
                .filter_map(|attachment_frame| {
                    match attachment_frame.kind() {
                        FrameKind::Attachment(AttachmentKind::Printable(attachment)) => {
                            // Any Display
                            Some(format!("{attachment}"))
                        }
                        _ => None, // Ignore opaque and context
                    }
                })
                .collect();

            let sources_vec = frame
                .sources()
                .iter()
                .flat_map(|source| into_structured_error_from_frame(source, &[]))
                .collect();

            vec![StructuredError {
                message: format!("{context}"),
                attachments: attachments_msgs,
                sources: sources_vec,
            }]
        }
        FrameKind::Attachment(_) => match frame.sources() {
            [] => {
                // Theoretically shouldn't happen, but return empty
                unreachable!("Attachment frame should not have no sources");
            }
            [source] => {
                attachments.push(frame);
                // Continue down single-child
                into_structured_error_from_frame(source, &attachments)
            }
            sources => {
                attachments.push(frame);
                // For each fork, clone attachments, flat_map results
                sources
                    .iter()
                    .flat_map(|source| into_structured_error_from_frame(source, &attachments))
                    .collect()
            }
        },
    }
}

impl<E: ThreadSafeError> From<Report<E>> for StructuredError {
    fn from(value: Report<E>) -> Self {
        // Top-level sources: flatten/collect
        let error: Vec<StructuredError> =
            into_structured_error_from_frame(value.current_frame(), &[]);

        // Should always yield one at top level. Fall back to dummy if not.
        error
            .into_iter()
            .next()
            .expect("Expected at least one top level error context in report")
    }
}
