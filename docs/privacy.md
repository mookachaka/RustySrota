# Privacy and Threat Model

## Purpose
Document the privacy goals and the protections applied to shared gesture descriptors.

## Threat model
- **Adversary**: server operator or external attacker who receives shared descriptors.
- **Assets to protect**: raw stroke coordinates, timing, and any metadata that can identify a user or reveal sensitive content.
- **Assumptions**: device can compute descriptors locally; communication channel may be observed.

## Protections applied
- **Invariant descriptor**: descriptors intentionally drop absolute position and orientation by resampling and using rotation/translation invariant features (curvature histogram, Fourier magnitudes).
- **Dimensionality reduction**: descriptor is a compact vector that omits raw coordinates.
- **Optional differential privacy**: add calibrated noise to descriptor components before sharing. Choose epsilon according to your risk tolerance; typical starting values for experimentation: epsilon = 1.0 (strong privacy) to 10.0 (weaker privacy).
- **On-device inference**: prefer running models locally so raw strokes never leave the device. Use federated learning for server-side model updates without sharing raw data.

## Practical guidance
- Evaluate utility vs privacy by sweeping epsilon and measuring classification accuracy.
- Attempt naive reconstruction attacks on descriptors during testing; if reconstruction succeeds, increase noise or reduce descriptor fidelity.
- Document the chosen epsilon and rationale in the repository for auditability.