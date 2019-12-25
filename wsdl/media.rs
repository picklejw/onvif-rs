<?xml version="1.0" encoding="utf-8"?>
<?xml-stylesheet type="text/xsl" href="../../../ver20/util/onvif-wsdl-viewer.xsl"?>
<!--
Copyright (c) 2008-2019 by ONVIF: Open Network Video Interface Forum. All rights reserved.

Recipients of this document may copy, distribute, publish, or display this document so long as this copyright notice, license and disclaimer are retained with all copies of the document. No license is granted to modify this document.

THIS DOCUMENT IS PROVIDED "AS IS," AND THE CORPORATION AND ITS MEMBERS AND THEIR AFFILIATES, MAKE NO REPRESENTATIONS OR WARRANTIES, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO, WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE, NON-INFRINGEMENT, OR TITLE; THAT THE CONTENTS OF THIS DOCUMENT ARE SUITABLE FOR ANY PURPOSE; OR THAT THE IMPLEMENTATION OF SUCH CONTENTS WILL NOT INFRINGE ANY PATENTS, COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS.
IN NO EVENT WILL THE CORPORATION OR ITS MEMBERS OR THEIR AFFILIATES BE LIABLE FOR ANY DIRECT, INDIRECT, SPECIAL, INCIDENTAL, PUNITIVE OR CONSEQUENTIAL DAMAGES, ARISING OUT OF OR RELATING TO ANY USE OR DISTRIBUTION OF THIS DOCUMENT, WHETHER OR NOT (1) THE CORPORATION, MEMBERS OR THEIR AFFILIATES HAVE BEEN ADVISED OF THE POSSIBILITY OF SUCH DAMAGES, OR (2) SUCH DAMAGES WERE REASONABLY FORESEEABLE, AND ARISING OUT OF OR RELATING TO ANY USE OR DISTRIBUTION OF THIS DOCUMENT.  THE FOREGOING DISCLAIMER AND LIMITATION ON LIABILITY DO NOT APPLY TO, INVALIDATE, OR LIMIT REPRESENTATIONS AND WARRANTIES MADE BY THE MEMBERS AND THEIR RESPECTIVE AFFILIATES TO THE CORPORATION AND OTHER MEMBERS IN CERTAIN WRITTEN POLICIES OF THE CORPORATION.
-->
<wsdl:definitions xmlns:wsdl="http://schemas.xmlsoap.org/wsdl/" xmlns:soap="http://schemas.xmlsoap.org/wsdl/soap12/" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:trt="http://www.onvif.org/ver10/media/wsdl" targetNamespace="http://www.onvif.org/ver10/media/wsdl">
	<wsdl:types>
		<xs:schema targetNamespace="http://www.onvif.org/ver10/media/wsdl" xmlns:tt="http://www.onvif.org/ver10/schema" xmlns:xs="http://www.w3.org/2001/XMLSchema" elementFormDefault="qualified" version="19.06">
			<xs:import namespace="http://www.onvif.org/ver10/schema" schemaLocation="../../../ver10/schema/onvif.xsd"/>
			<!--  Message Request/Responses elements  -->
			<!--===============================-->
			<xs:element name="GetServiceCapabilities">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetServiceCapabilitiesResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Capabilities" type="trt:Capabilities">
							<xs:annotation>
								<xs:documentation>The capabilities for the media service is returned in the Capabilities element.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:complexType name="Capabilities">
				<xs:sequence>
					<xs:element name="ProfileCapabilities" type="trt:ProfileCapabilities">
						<xs:annotation>
							<xs:documentation>Media profile capabilities.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="StreamingCapabilities" type="trt:StreamingCapabilities">
						<xs:annotation>
							<xs:documentation>Streaming capabilities.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>	 <!-- first Vendor then ONVIF -->
				</xs:sequence>
				<xs:attribute name="SnapshotUri" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates if GetSnapshotUri is supported.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="Rotation" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates whether or not Rotation feature is supported.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="VideoSourceMode" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates the support for changing video source mode.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="OSD" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates if OSD is supported.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="TemporaryOSDText" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates the support for temporary osd text configuration. </xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="EXICompression" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates the support for the Efficient XML Interchange (EXI) binary XML format.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<xs:element name="Capabilities" type="trt:Capabilities"/>
			<!--===============================-->
			<xs:complexType name="ProfileCapabilities">
				<xs:sequence>
					<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>	 <!-- first Vendor then ONVIF -->
				</xs:sequence>
				<xs:attribute name="MaximumNumberOfProfiles" type="xs:int">
					<xs:annotation>
						<xs:documentation>Maximum number of profiles supported.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<!--===============================-->
			<xs:complexType name="StreamingCapabilities">
				<xs:sequence>
					<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>	 <!-- first Vendor then ONVIF -->
				</xs:sequence>
				<xs:attribute name="RTPMulticast" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for RTP multicast.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="RTP_TCP" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for RTP over TCP.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="RTP_RTSP_TCP" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for RTP/RTSP/TCP.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="NonAggregateControl" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for non aggregate RTSP control.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="NoRTSPStreaming" type="xs:boolean">
					<xs:annotation>
						<xs:documentation> Indicates the device does not support live media streaming via RTSP.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<!--===============================-->
			<xs:element name="GetVideoSources">
				<xs:complexType>
					<xs:sequence>
				   </xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetVideoSourcesResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="VideoSources" type="tt:VideoSource" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>List of existing Video Sources</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetAudioSources">
				<xs:complexType>
					<xs:sequence>
          </xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetAudioSourcesResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="AudioSources" type="tt:AudioSource" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>List of existing Audio Sources</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetAudioOutputs">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetAudioOutputsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="AudioOutputs" type="tt:AudioOutput" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>List of existing Audio Outputs</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="CreateProfile">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Name" type="tt:Name">
							<xs:annotation>
								<xs:documentation>friendly name of the profile to be created</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="Token" type="tt:ReferenceToken" minOccurs="0">
							<xs:annotation>
								<xs:documentation>Optional token, specifying the unique identifier of the new profile. <br/>A device supports at least a token length of 12 characters and characters "A-Z" | "a-z" | "0-9" | "-.".</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="CreateProfileResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Profile" type="tt:Profile">
							<xs:annotation>
								<xs:documentation>returns the new created profile</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetProfile">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>this command requests a specific profile</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetProfileResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Profile" type="tt:Profile">
							<xs:annotation>
								<xs:documentation>returns the requested media profile</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetProfiles">
				<xs:complexType>
					<xs:sequence>
				
      </xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetProfilesResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Profiles" type="tt:Profile" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>lists all profiles that exist in the media service </xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="AddVideoEncoderConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Reference to the profile where the configuration should be added</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="ConfigurationToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Contains a reference to the VideoEncoderConfiguration to add</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="AddVideoEncoderConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
				   </xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="RemoveVideoEncoderConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Contains a reference to the media profile from which the
VideoEncoderConfiguration shall be removed.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="RemoveVideoEncoderConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
    </xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="AddVideoSourceConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Reference to the profile where the configuration should be added</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="ConfigurationToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Contains a reference to the VideoSourceConfiguration to add</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="AddVideoSourceConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
          </xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="RemoveVideoSourceConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Contains a reference to the media profile from which the
VideoSourceConfiguration shall be removed.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="RemoveVideoSourceConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
          </xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="AddAudioEncoderConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Reference to the profile where the configuration should be added</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="ConfigurationToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Contains a reference to the AudioEncoderConfiguration to add</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="AddAudioEncoderConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
          </xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="RemoveAudioEncoderConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Contains a reference to the media profile from which the
AudioEncoderConfiguration shall be removed.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="RemoveAudioEncoderConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
          </xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="AddAudioSourceConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Reference to the profile where the configuration should be added</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="ConfigurationToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Contains a reference to the AudioSourceConfiguration to add</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="AddAudioSourceConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
          </xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="RemoveAudioSourceConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Contains a reference to the media profile from which the
AudioSourceConfiguration shall be removed.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="RemoveAudioSourceConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
          </xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="AddPTZConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Reference to the profile where the configuration should be added</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="ConfigurationToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Contains a reference to the PTZConfiguration to add</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="AddPTZConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
          </xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="RemovePTZConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Contains a reference to the media profile from which the
PTZConfiguration shall be removed.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="RemovePTZConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
          </xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="AddVideoAnalyticsConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Reference to the profile where the configuration should be added</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="ConfigurationToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Contains a reference to the VideoAnalyticsConfiguration to add</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="AddVideoAnalyticsConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
          </xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="RemoveVideoAnalyticsConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Contains a reference to the media profile from which the
VideoAnalyticsConfiguration shall be removed.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="RemoveVideoAnalyticsConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
          </xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="AddMetadataConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Reference to the profile where the configuration should be added</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="ConfigurationToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Contains a reference to the MetadataConfiguration to add</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="AddMetadataConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
          </xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="RemoveMetadataConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Contains a reference to the media profile from which the
MetadataConfiguration shall be removed.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="RemoveMetadataConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
          </xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="AddAudioOutputConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Reference to the profile where the configuration should be added</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="ConfigurationToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Contains a reference to the AudioOutputConfiguration to add</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="AddAudioOutputConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
                   </xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="RemoveAudioOutputConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Contains a reference to the media profile from which the
AudioOutputConfiguration shall be removed.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="RemoveAudioOutputConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
                    </xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="AddAudioDecoderConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>This element contains a reference to the profile where the configuration should be added.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="ConfigurationToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>This element contains a reference to the AudioDecoderConfiguration to add.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="AddAudioDecoderConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
                    </xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="RemoveAudioDecoderConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>This element contains a  reference to the media profile from which the AudioDecoderConfiguration shall be removed.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="RemoveAudioDecoderConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
                    </xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="DeleteProfile">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>This element contains a  reference to the profile that should be deleted.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="DeleteProfileResponse">
				<xs:complexType>
					<xs:sequence>
          </xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<!--===============================-->
			<!--===============================-->
			<xs:element name="GetVideoEncoderConfigurations">
				<xs:complexType>
					<xs:sequence>
      </xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetVideoEncoderConfigurationsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configurations" type="tt:VideoEncoderConfiguration" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>This element contains a list of video encoder configurations.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetVideoSourceConfigurations">
				<xs:complexType>
					<xs:sequence>
          </xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetVideoSourceConfigurationsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configurations" type="tt:VideoSourceConfiguration" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>This element contains a list of video source configurations.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetAudioEncoderConfigurations">
				<xs:complexType>
					<xs:sequence>
          </xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetAudioEncoderConfigurationsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configurations" type="tt:AudioEncoderConfiguration" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>This element contains a list of audio encoder configurations.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetAudioSourceConfigurations">
				<xs:complexType>
					<xs:sequence>
      </xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetAudioSourceConfigurationsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configurations" type="tt:AudioSourceConfiguration" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>This element contains a list of audio source configurations.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetVideoAnalyticsConfigurations">
				<xs:complexType>
					<xs:sequence>
      </xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetVideoAnalyticsConfigurationsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configurations" type="tt:VideoAnalyticsConfiguration" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>This element contains a list of VideoAnalytics configurations.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetMetadataConfigurations">
				<xs:complexType>
					<xs:sequence>
      </xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetMetadataConfigurationsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configurations" type="tt:MetadataConfiguration" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>This element contains a list of metadata configurations</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<!--===============================-->
			<xs:element name="GetAudioOutputConfigurations">
				<xs:complexType>
					<xs:sequence>
				   </xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetAudioOutputConfigurationsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configurations" type="tt:AudioOutputConfiguration" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>This element contains a list of audio output configurations</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetAudioDecoderConfigurations">
				<xs:complexType>
					<xs:sequence>
				   </xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetAudioDecoderConfigurationsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configurations" type="tt:AudioDecoderConfiguration" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>This element contains a list of audio decoder configurations</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetVideoSourceConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ConfigurationToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Token of the requested video source configuration.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetVideoSourceConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configuration" type="tt:VideoSourceConfiguration">
							<xs:annotation>
								<xs:documentation>The requested video source configuration.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetVideoEncoderConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ConfigurationToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Token of the requested video encoder configuration.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetVideoEncoderConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configuration" type="tt:VideoEncoderConfiguration">
							<xs:annotation>
								<xs:documentation>The requested video encoder configuration.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetAudioSourceConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ConfigurationToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Token of the requested audio source configuration.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetAudioSourceConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configuration" type="tt:AudioSourceConfiguration">
							<xs:annotation>
								<xs:documentation>The requested audio source configuration.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetAudioEncoderConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ConfigurationToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Token of the requested audio encoder configuration.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetAudioEncoderConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configuration" type="tt:AudioEncoderConfiguration">
							<xs:annotation>
								<xs:documentation>The requested audio encoder configuration</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetVideoAnalyticsConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ConfigurationToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Token of the requested video analytics configuration.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetVideoAnalyticsConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configuration" type="tt:VideoAnalyticsConfiguration">
							<xs:annotation>
								<xs:documentation>The requested video analytics configuration.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetMetadataConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ConfigurationToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Token of the requested metadata configuration.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetMetadataConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configuration" type="tt:MetadataConfiguration">
							<xs:annotation>
								<xs:documentation>The requested metadata configuration.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<!--===============================-->
			<xs:element name="GetAudioOutputConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ConfigurationToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Token of the requested audio output configuration.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetAudioOutputConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configuration" type="tt:AudioOutputConfiguration">
							<xs:annotation>
								<xs:documentation>The requested audio output configuration.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetAudioDecoderConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ConfigurationToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Token of the requested audio decoder configuration.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetAudioDecoderConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configuration" type="tt:AudioDecoderConfiguration">
							<xs:annotation>
								<xs:documentation>The requested audio decoder configuration</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetCompatibleVideoEncoderConfigurations">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Contains the token of an existing media profile the configurations shall be compatible with.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetCompatibleVideoEncoderConfigurationsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configurations" type="tt:VideoEncoderConfiguration" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>Contains a list of video encoder configurations that are compatible with the specified media profile.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetCompatibleVideoSourceConfigurations">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Contains the token of an existing media profile the configurations shall be compatible with.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetCompatibleVideoSourceConfigurationsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configurations" type="tt:VideoSourceConfiguration" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>Contains a list of video source configurations that are compatible with the specified media profile.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetCompatibleAudioEncoderConfigurations">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Contains the token of an existing media profile the configurations shall be compatible with.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetCompatibleAudioEncoderConfigurationsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configurations" type="tt:AudioEncoderConfiguration" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>Contains a list of audio encoder configurations that are compatible with the specified media profile.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetCompatibleAudioSourceConfigurations">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Contains the token of an existing media profile the configurations shall be compatible with.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetCompatibleAudioSourceConfigurationsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configurations" type="tt:AudioSourceConfiguration" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>Contains a list of audio source configurations that are compatible with the specified media profile.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetCompatibleVideoAnalyticsConfigurations">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Contains the token of an existing media profile the configurations shall be compatible with.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetCompatibleVideoAnalyticsConfigurationsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configurations" type="tt:VideoAnalyticsConfiguration" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>Contains a list of video analytics configurations that are compatible with the specified media profile.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetCompatibleMetadataConfigurations">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Contains the token of an existing media profile the configurations shall be compatible with.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetCompatibleMetadataConfigurationsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configurations" type="tt:MetadataConfiguration" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>Contains a list of metadata configurations that are compatible with the specified media profile.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<!--===============================-->
			<xs:element name="GetCompatibleAudioOutputConfigurations">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Contains the token of an existing media profile the configurations shall be compatible with.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetCompatibleAudioOutputConfigurationsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configurations" type="tt:AudioOutputConfiguration" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>Contains a list of audio output configurations that are compatible with the specified media profile.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetCompatibleAudioDecoderConfigurations">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Contains the token of an existing media profile the configurations shall be compatible with.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetCompatibleAudioDecoderConfigurationsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configurations" type="tt:AudioDecoderConfiguration" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>Contains a list of audio decoder configurations that are compatible with the specified media profile. </xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<!--===============================-->
			<!--===============================-->
			<!--===============================-->
			<!--===============================-->
			<!--===============================-->
			<xs:element name="SetVideoEncoderConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configuration" type="tt:VideoEncoderConfiguration">
							<xs:annotation>
								<xs:documentation>Contains the modified video encoder configuration. The configuration shall exist in the device.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="ForcePersistence" type="xs:boolean">
							<xs:annotation>
								<xs:documentation>The ForcePersistence element is obsolete and should always be assumed to be true.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetVideoEncoderConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
          </xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetVideoSourceConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configuration" type="tt:VideoSourceConfiguration">
							<xs:annotation>
								<xs:documentation>Contains the modified video source configuration. The configuration shall exist in the device.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="ForcePersistence" type="xs:boolean">
							<xs:annotation>
								<xs:documentation>The ForcePersistence element is obsolete and should always be assumed to be true.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetVideoSourceConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
          </xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetAudioEncoderConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configuration" type="tt:AudioEncoderConfiguration">
							<xs:annotation>
								<xs:documentation>Contains the modified audio encoder configuration. The configuration shall exist in the device.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="ForcePersistence" type="xs:boolean">
							<xs:annotation>
								<xs:documentation>The ForcePersistence element is obsolete and should always be assumed to be true.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetAudioEncoderConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
          </xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetAudioSourceConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configuration" type="tt:AudioSourceConfiguration">
							<xs:annotation>
								<xs:documentation>Contains the modified audio source configuration. The configuration shall exist in the device.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="ForcePersistence" type="xs:boolean">
							<xs:annotation>
								<xs:documentation>The ForcePersistence element is obsolete and should always be assumed to be true.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetAudioSourceConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
          </xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetVideoAnalyticsConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configuration" type="tt:VideoAnalyticsConfiguration">
							<xs:annotation>
								<xs:documentation>Contains the modified video analytics configuration. The configuration shall exist in the device.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="ForcePersistence" type="xs:boolean">
							<xs:annotation>
								<xs:documentation>The ForcePersistence element is obsolete and should always be assumed to be true.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetVideoAnalyticsConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
          </xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetMetadataConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configuration" type="tt:MetadataConfiguration">
							<xs:annotation>
								<xs:documentation>Contains the modified metadata configuration. The configuration shall exist in the device.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="ForcePersistence" type="xs:boolean">
							<xs:annotation>
								<xs:documentation>The ForcePersistence element is obsolete and should always be assumed to be true.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetMetadataConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
          </xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<!--===============================-->
			<xs:element name="SetAudioOutputConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configuration" type="tt:AudioOutputConfiguration">
							<xs:annotation>
								<xs:documentation>Contains the modified audio output configuration. The configuration shall exist in the device.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="ForcePersistence" type="xs:boolean">
							<xs:annotation>
								<xs:documentation>The ForcePersistence element is obsolete and should always be assumed to be true.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetAudioOutputConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
				   </xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetAudioDecoderConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configuration" type="tt:AudioDecoderConfiguration">
							<xs:annotation>
								<xs:documentation>Contains the modified audio decoder configuration. The configuration shall exist in the device.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="ForcePersistence" type="xs:boolean">
							<xs:annotation>
								<xs:documentation>The ForcePersistence element is obsolete and should always be assumed to be true.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetAudioDecoderConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
				   </xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetVideoSourceConfigurationOptions">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ConfigurationToken" type="tt:ReferenceToken" minOccurs="0">
							<xs:annotation>
								<xs:documentation>Optional video source configurationToken that specifies an existing configuration that the options are intended for.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="ProfileToken" type="tt:ReferenceToken" minOccurs="0">
							<xs:annotation>
								<xs:documentation>Optional ProfileToken that specifies an existing media profile that the options shall be compatible with.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetVideoSourceConfigurationOptionsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Options" type="tt:VideoSourceConfigurationOptions">
							<xs:annotation>
								<xs:documentation>This message contains the video source configuration options. If a video source configuration is specified, the options shall concern that particular configuration. If a media profile is specified, the options shall be compatible with that media profile. If no tokens are specified, the options shall be considered generic for the device.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetVideoEncoderConfigurationOptions">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ConfigurationToken" type="tt:ReferenceToken" minOccurs="0">
							<xs:annotation>
								<xs:documentation>Optional video encoder configuration token that specifies an existing configuration that the options are intended for.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="ProfileToken" type="tt:ReferenceToken" minOccurs="0">
							<xs:annotation>
								<xs:documentation>Optional ProfileToken that specifies an existing media profile that the options shall be compatible with.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetVideoEncoderConfigurationOptionsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Options" type="tt:VideoEncoderConfigurationOptions"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetAudioSourceConfigurationOptions">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ConfigurationToken" type="tt:ReferenceToken" minOccurs="0">
							<xs:annotation>
								<xs:documentation>Optional audio source configuration token that specifies an existing configuration that the options are intended for.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="ProfileToken" type="tt:ReferenceToken" minOccurs="0">
							<xs:annotation>
								<xs:documentation>Optional ProfileToken that specifies an existing media profile that the options shall be compatible with.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetAudioSourceConfigurationOptionsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Options" type="tt:AudioSourceConfigurationOptions">
							<xs:annotation>
								<xs:documentation>This message contains the audio source configuration options. If a audio source configuration is specified, the options shall concern that particular configuration. If a media profile is specified, the options shall be compatible with that media profile. If no tokens are specified, the options shall be considered generic for the device.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetAudioEncoderConfigurationOptions">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ConfigurationToken" type="tt:ReferenceToken" minOccurs="0">
							<xs:annotation>
								<xs:documentation>Optional audio encoder configuration token that specifies an existing configuration that the options are intended for.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="ProfileToken" type="tt:ReferenceToken" minOccurs="0">
							<xs:annotation>
								<xs:documentation>Optional ProfileToken that specifies an existing media profile that the options shall be compatible with.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetAudioEncoderConfigurationOptionsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Options" type="tt:AudioEncoderConfigurationOptions">
							<xs:annotation>
								<xs:documentation>This message contains the audio encoder configuration options. If a audio encoder configuration is specified, the options shall concern that particular configuration. If a media profile is specified, the options shall be compatible with that media profile. If no tokens are specified, the options shall be considered generic for the device.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetMetadataConfigurationOptions">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ConfigurationToken" type="tt:ReferenceToken" minOccurs="0">
							<xs:annotation>
								<xs:documentation>Optional metadata configuration token that specifies an existing configuration that the options are intended for.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="ProfileToken" type="tt:ReferenceToken" minOccurs="0">
							<xs:annotation>
								<xs:documentation>Optional ProfileToken that specifies an existing media profile that the options shall be compatible with.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetMetadataConfigurationOptionsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Options" type="tt:MetadataConfigurationOptions">
							<xs:annotation>
								<xs:documentation>This message contains the metadata configuration options. If a metadata configuration is specified, the options shall concern that particular configuration. If a media profile is specified, the options shall be compatible with that media profile. If no tokens are specified, the options shall be considered generic for the device.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetAudioOutputConfigurationOptions">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ConfigurationToken" type="tt:ReferenceToken" minOccurs="0">
							<xs:annotation>
								<xs:documentation>Optional audio output configuration token that specifies an existing configuration that the options are intended for.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="ProfileToken" type="tt:ReferenceToken" minOccurs="0">
							<xs:annotation>
								<xs:documentation>Optional ProfileToken that specifies an existing media profile that the options shall be compatible with.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetAudioOutputConfigurationOptionsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Options" type="tt:AudioOutputConfigurationOptions">
							<xs:annotation>
								<xs:documentation>This message contains the audio output configuration options. If a audio output configuration is specified, the options shall concern that particular configuration. If a media profile is specified, the options shall be compatible with that media profile. If no tokens are specified, the options shall be considered generic for the device.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetAudioDecoderConfigurationOptions">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ConfigurationToken" type="tt:ReferenceToken" minOccurs="0">
							<xs:annotation>
								<xs:documentation>Optional audio decoder configuration token that specifies an existing configuration that the options are intended for.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="ProfileToken" type="tt:ReferenceToken" minOccurs="0">
							<xs:annotation>
								<xs:documentation>Optional ProfileToken that specifies an existing media profile that the options shall be compatible with.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetAudioDecoderConfigurationOptionsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Options" type="tt:AudioDecoderConfigurationOptions">
							<xs:annotation>
								<xs:documentation>This message contains the audio decoder configuration options. If a audio decoder configuration is specified, the options shall concern that particular configuration. If a media profile is specified, the options shall be compatible with that media profile. If no tokens are specified, the options shall be considered generic for the device.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetGuaranteedNumberOfVideoEncoderInstances">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ConfigurationToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Token of the video source configuration</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetGuaranteedNumberOfVideoEncoderInstancesResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="TotalNumber" type="xs:int">
							<xs:annotation>
								<xs:documentation>The minimum guaranteed total number of encoder instances (applications) per VideoSourceConfiguration. The device is able to deliver the TotalNumber of streams</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="JPEG" type="xs:int" minOccurs="0">
							<xs:annotation>
								<xs:documentation>If a device limits the number of instances for respective Video Codecs the response contains the information how many Jpeg streams can be set up at the same time per VideoSource.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="H264" type="xs:int" minOccurs="0">
							<xs:annotation>
								<xs:documentation>If a device limits the number of instances for respective Video Codecs the response contains the information how many H264 streams can be set up at the same time per VideoSource.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="MPEG4" type="xs:int" minOccurs="0">
							<xs:annotation>
								<xs:documentation>If a device limits the number of instances for respective Video Codecs the response contains the information how many Mpeg4 streams can be set up at the same time per VideoSource.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetStreamUri">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="StreamSetup" type="tt:StreamSetup">
							<xs:annotation>
								<xs:documentation>Stream Setup that should be used with the uri</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>The ProfileToken element indicates the media profile to use and will define the configuration of the content of the stream.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetStreamUriResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="MediaUri" type="tt:MediaUri">
							<xs:annotation>
								<xs:documentation/>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="StartMulticastStreaming">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Contains the token of the Profile that is used to define the multicast stream.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="StartMulticastStreamingResponse">
				<xs:complexType>
					<xs:sequence>
          </xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="StopMulticastStreaming">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Contains the token of the Profile that is used to define the multicast stream.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="StopMulticastStreamingResponse">
				<xs:complexType>
					<xs:sequence>
          </xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetSynchronizationPoint">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Contains a Profile reference for which a Synchronization Point is requested.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetSynchronizationPointResponse">
				<xs:complexType>
					<xs:sequence>
          </xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetSnapshotUri">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>The ProfileToken element indicates the media profile to use and will define the source and dimensions of the snapshot.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetSnapshotUriResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="MediaUri" type="tt:MediaUri">
							<xs:annotation>
								<xs:documentation/>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--================ Video Source Mode ===============-->
			<xs:element name="GetVideoSourceModes">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="VideoSourceToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Contains a video source reference for which a video source mode is requested.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetVideoSourceModesResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="VideoSourceModes" type="trt:VideoSourceMode" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>Return the information for specified video source mode.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetVideoSourceMode">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="VideoSourceToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Contains a video source reference for which a video source mode is requested.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="VideoSourceModeToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Indicate video source mode.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetVideoSourceModeResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Reboot" type="xs:boolean">
							<xs:annotation>
								<xs:documentation>The response contains information about rebooting after returning response. When Reboot is set true, a device will reboot automatically after setting mode.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:simpleType name="EncodingTypes">
				<xs:annotation>
					<xs:documentation>Indication which encodings are supported for this video source. The list may contain one or more enumeration values of tt:VideoEncoding.</xs:documentation>
				</xs:annotation>
				<xs:list itemType="xs:string"/>
			</xs:simpleType>
			<!--===============================-->
			<xs:complexType name="VideoSourceMode"> 
				<xs:sequence>
					<xs:element name="MaxFramerate" type="xs:float">
						<xs:annotation>
							<xs:documentation>Max frame rate in frames per second for this video source mode.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="MaxResolution" type="tt:VideoResolution">
						<xs:annotation>
							<xs:documentation>Max horizontal and vertical resolution for this video source mode.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="Encodings" type="trt:EncodingTypes">
						<xs:annotation>
							<xs:documentation>Indication which encodings are supported for this video source. The list may contain one or more enumeration values of tt:VideoEncoding.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="Reboot" type="xs:boolean">
						<xs:annotation>
							<xs:documentation>After setting the mode if a device starts to reboot this value is true. If a device change the mode without rebooting this value is false. If true, configured parameters may not be guaranteed by the device after rebooting.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="Description" type="tt:Description" minOccurs="0">
						<xs:annotation>
							<xs:documentation>Informative description of this video source mode. This field should be described in English.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="Extension" type="trt:VideoSourceModeExtension" minOccurs="0"/>
				</xs:sequence>
				<xs:attribute name="token" type="tt:ReferenceToken" use="required">
					<xs:annotation>
						<xs:documentation>Indicate token for video source mode.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="Enabled" type="xs:boolean"> 
					<xs:annotation>
						<xs:documentation>Indication of whether this mode is active. If active this value is true. In case of non-indication, it means as false. The value of true shall be had by only one video source mode.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<!--===============================-->
			<xs:complexType name="VideoSourceModeExtension"> 
				<xs:sequence>
					<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>	 <!-- first Vendor then ONVIF -->
				</xs:sequence>
			</xs:complexType>

			<!--===============================-->
			<!--============OSD Schema Begin================-->
			<xs:element name="GetOSDs">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ConfigurationToken" type="tt:ReferenceToken" minOccurs="0" maxOccurs="1">
							<xs:annotation>
								<xs:documentation>Token of the Video Source Configuration, which has OSDs associated with are requested. If token not exist, request all available OSDs.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetOSDsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="OSDs" type="tt:OSDConfiguration" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>This element contains a list of requested OSDs.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetOSD">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="OSDToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>The GetOSD command fetches the OSD configuration if the OSD token is known.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>	 <!-- first Vendor then ONVIF -->
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetOSDResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="OSD" type="tt:OSDConfiguration">
							<xs:annotation>
								<xs:documentation>The requested OSD configuration.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>	 <!-- first Vendor then ONVIF -->
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetOSD">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="OSD" type="tt:OSDConfiguration">
							<xs:annotation>
								<xs:documentation>Contains the modified OSD configuration.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>	 <!-- first Vendor then ONVIF -->
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetOSDResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>	 <!-- first Vendor then ONVIF -->
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetOSDOptions">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ConfigurationToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Video Source Configuration Token that specifies an existing video source configuration that the options shall be compatible with.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>	 <!-- first Vendor then ONVIF -->
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetOSDOptionsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="OSDOptions" type="tt:OSDConfigurationOptions">
							<xs:annotation>
								<xs:documentation/>
							</xs:annotation>
						</xs:element>
						<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>	 <!-- first Vendor then ONVIF -->
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="CreateOSD">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="OSD" type="tt:OSDConfiguration">
							<xs:annotation>
								<xs:documentation>Contain the initial OSD configuration for create.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>	 <!-- first Vendor then ONVIF -->
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="CreateOSDResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="OSDToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Returns Token of the newly created OSD</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>	 <!-- first Vendor then ONVIF -->
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="DeleteOSD">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="OSDToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>This element contains a reference to the OSD configuration that should be deleted.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>	 <!-- first Vendor then ONVIF -->
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="DeleteOSDResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>	 <!-- first Vendor then ONVIF -->
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--============OSD Schema End================-->
		</xs:schema>
	</wsdl:types>
	<wsdl:message name="GetServiceCapabilitiesRequest">
		<wsdl:part name="parameters" element="trt:GetServiceCapabilities"/>
	</wsdl:message>
	<wsdl:message name="GetServiceCapabilitiesResponse">
		<wsdl:part name="parameters" element="trt:GetServiceCapabilitiesResponse"/>
	</wsdl:message>
	<wsdl:message name="GetVideoSourcesRequest">
		<wsdl:part name="parameters" element="trt:GetVideoSources"/>
	</wsdl:message>
	<wsdl:message name="GetVideoSourcesResponse">
		<wsdl:part name="parameters" element="trt:GetVideoSourcesResponse"/>
	</wsdl:message>
	<wsdl:message name="GetAudioSourcesRequest">
		<wsdl:part name="parameters" element="trt:GetAudioSources"/>
	</wsdl:message>
	<wsdl:message name="GetAudioSourcesResponse">
		<wsdl:part name="parameters" element="trt:GetAudioSourcesResponse"/>
	</wsdl:message>
	<wsdl:message name="GetAudioOutputsRequest">
		<wsdl:part name="parameters" element="trt:GetAudioOutputs"/>
	</wsdl:message>
	<wsdl:message name="GetAudioOutputsResponse">
		<wsdl:part name="parameters" element="trt:GetAudioOutputsResponse"/>
	</wsdl:message>
	<wsdl:message name="CreateProfileRequest">
		<wsdl:part name="parameters" element="trt:CreateProfile"/>
	</wsdl:message>
	<wsdl:message name="CreateProfileResponse">
		<wsdl:part name="parameters" element="trt:CreateProfileResponse"/>
	</wsdl:message>
	<wsdl:message name="GetProfileRequest">
		<wsdl:part name="parameters" element="trt:GetProfile"/>
	</wsdl:message>
	<wsdl:message name="GetProfileResponse">
		<wsdl:part name="parameters" element="trt:GetProfileResponse"/>
	</wsdl:message>
	<wsdl:message name="GetProfilesRequest">
		<wsdl:part name="parameters" element="trt:GetProfiles"/>
	</wsdl:message>
	<wsdl:message name="GetProfilesResponse">
		<wsdl:part name="parameters" element="trt:GetProfilesResponse"/>
	</wsdl:message>
	<wsdl:message name="AddVideoEncoderConfigurationRequest">
		<wsdl:part name="parameters" element="trt:AddVideoEncoderConfiguration"/>
	</wsdl:message>
	<wsdl:message name="AddVideoEncoderConfigurationResponse">
		<wsdl:part name="parameters" element="trt:AddVideoEncoderConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="RemoveVideoEncoderConfigurationRequest">
		<wsdl:part name="parameters" element="trt:RemoveVideoEncoderConfiguration"/>
	</wsdl:message>
	<wsdl:message name="RemoveVideoEncoderConfigurationResponse">
		<wsdl:part name="parameters" element="trt:RemoveVideoEncoderConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="AddVideoSourceConfigurationRequest">
		<wsdl:part name="parameters" element="trt:AddVideoSourceConfiguration"/>
	</wsdl:message>
	<wsdl:message name="AddVideoSourceConfigurationResponse">
		<wsdl:part name="parameters" element="trt:AddVideoSourceConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="RemoveVideoSourceConfigurationRequest">
		<wsdl:part name="parameters" element="trt:RemoveVideoSourceConfiguration"/>
	</wsdl:message>
	<wsdl:message name="RemoveVideoSourceConfigurationResponse">
		<wsdl:part name="parameters" element="trt:RemoveVideoSourceConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="AddAudioEncoderConfigurationRequest">
		<wsdl:part name="parameters" element="trt:AddAudioEncoderConfiguration"/>
	</wsdl:message>
	<wsdl:message name="AddAudioEncoderConfigurationResponse">
		<wsdl:part name="parameters" element="trt:AddAudioEncoderConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="RemoveAudioEncoderConfigurationRequest">
		<wsdl:part name="parameters" element="trt:RemoveAudioEncoderConfiguration"/>
	</wsdl:message>
	<wsdl:message name="RemoveAudioEncoderConfigurationResponse">
		<wsdl:part name="parameters" element="trt:RemoveAudioEncoderConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="AddAudioSourceConfigurationRequest">
		<wsdl:part name="parameters" element="trt:AddAudioSourceConfiguration"/>
	</wsdl:message>
	<wsdl:message name="AddAudioSourceConfigurationResponse">
		<wsdl:part name="parameters" element="trt:AddAudioSourceConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="RemoveAudioSourceConfigurationRequest">
		<wsdl:part name="parameters" element="trt:RemoveAudioSourceConfiguration"/>
	</wsdl:message>
	<wsdl:message name="RemoveAudioSourceConfigurationResponse">
		<wsdl:part name="parameters" element="trt:RemoveAudioSourceConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="AddPTZConfigurationRequest">
		<wsdl:part name="parameters" element="trt:AddPTZConfiguration"/>
	</wsdl:message>
	<wsdl:message name="AddPTZConfigurationResponse">
		<wsdl:part name="parameters" element="trt:AddPTZConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="RemovePTZConfigurationRequest">
		<wsdl:part name="parameters" element="trt:RemovePTZConfiguration"/>
	</wsdl:message>
	<wsdl:message name="RemovePTZConfigurationResponse">
		<wsdl:part name="parameters" element="trt:RemovePTZConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="AddVideoAnalyticsConfigurationRequest">
		<wsdl:part name="parameters" element="trt:AddVideoAnalyticsConfiguration"/>
	</wsdl:message>
	<wsdl:message name="AddVideoAnalyticsConfigurationResponse">
		<wsdl:part name="parameters" element="trt:AddVideoAnalyticsConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="RemoveVideoAnalyticsConfigurationRequest">
		<wsdl:part name="parameters" element="trt:RemoveVideoAnalyticsConfiguration"/>
	</wsdl:message>
	<wsdl:message name="RemoveVideoAnalyticsConfigurationResponse">
		<wsdl:part name="parameters" element="trt:RemoveVideoAnalyticsConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="AddMetadataConfigurationRequest">
		<wsdl:part name="parameters" element="trt:AddMetadataConfiguration"/>
	</wsdl:message>
	<wsdl:message name="AddMetadataConfigurationResponse">
		<wsdl:part name="parameters" element="trt:AddMetadataConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="RemoveMetadataConfigurationRequest">
		<wsdl:part name="parameters" element="trt:RemoveMetadataConfiguration"/>
	</wsdl:message>
	<wsdl:message name="RemoveMetadataConfigurationResponse">
		<wsdl:part name="parameters" element="trt:RemoveMetadataConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="AddAudioOutputConfigurationRequest">
		<wsdl:part name="parameters" element="trt:AddAudioOutputConfiguration"/>
	</wsdl:message>
	<wsdl:message name="AddAudioOutputConfigurationResponse">
		<wsdl:part name="parameters" element="trt:AddAudioOutputConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="RemoveAudioOutputConfigurationRequest">
		<wsdl:part name="parameters" element="trt:RemoveAudioOutputConfiguration"/>
	</wsdl:message>
	<wsdl:message name="RemoveAudioOutputConfigurationResponse">
		<wsdl:part name="parameters" element="trt:RemoveAudioOutputConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="AddAudioDecoderConfigurationRequest">
		<wsdl:part name="parameters" element="trt:AddAudioDecoderConfiguration"/>
	</wsdl:message>
	<wsdl:message name="AddAudioDecoderConfigurationResponse">
		<wsdl:part name="parameters" element="trt:AddAudioDecoderConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="RemoveAudioDecoderConfigurationRequest">
		<wsdl:part name="parameters" element="trt:RemoveAudioDecoderConfiguration"/>
	</wsdl:message>
	<wsdl:message name="RemoveAudioDecoderConfigurationResponse">
		<wsdl:part name="parameters" element="trt:RemoveAudioDecoderConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="DeleteProfileRequest">
		<wsdl:part name="parameters" element="trt:DeleteProfile"/>
	</wsdl:message>
	<wsdl:message name="DeleteProfileResponse">
		<wsdl:part name="parameters" element="trt:DeleteProfileResponse"/>
	</wsdl:message>
	<wsdl:message name="GetVideoSourceConfigurationsRequest">
		<wsdl:part name="parameters" element="trt:GetVideoSourceConfigurations"/>
	</wsdl:message>
	<wsdl:message name="GetVideoSourceConfigurationsResponse">
		<wsdl:part name="parameters" element="trt:GetVideoSourceConfigurationsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetVideoEncoderConfigurationsRequest">
		<wsdl:part name="parameters" element="trt:GetVideoEncoderConfigurations"/>
	</wsdl:message>
	<wsdl:message name="GetVideoEncoderConfigurationsResponse">
		<wsdl:part name="parameters" element="trt:GetVideoEncoderConfigurationsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetAudioSourceConfigurationsRequest">
		<wsdl:part name="parameters" element="trt:GetAudioSourceConfigurations"/>
	</wsdl:message>
	<wsdl:message name="GetAudioSourceConfigurationsResponse">
		<wsdl:part name="parameters" element="trt:GetAudioSourceConfigurationsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetAudioEncoderConfigurationsRequest">
		<wsdl:part name="parameters" element="trt:GetAudioEncoderConfigurations"/>
	</wsdl:message>
	<wsdl:message name="GetAudioEncoderConfigurationsResponse">
		<wsdl:part name="parameters" element="trt:GetAudioEncoderConfigurationsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetVideoAnalyticsConfigurationsRequest">
		<wsdl:part name="parameters" element="trt:GetVideoAnalyticsConfigurations"/>
	</wsdl:message>
	<wsdl:message name="GetVideoAnalyticsConfigurationsResponse">
		<wsdl:part name="parameters" element="trt:GetVideoAnalyticsConfigurationsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetMetadataConfigurationsRequest">
		<wsdl:part name="parameters" element="trt:GetMetadataConfigurations"/>
	</wsdl:message>
	<wsdl:message name="GetMetadataConfigurationsResponse">
		<wsdl:part name="parameters" element="trt:GetMetadataConfigurationsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetAudioOutputConfigurationsRequest">
		<wsdl:part name="parameters" element="trt:GetAudioOutputConfigurations"/>
	</wsdl:message>
	<wsdl:message name="GetAudioOutputConfigurationsResponse">
		<wsdl:part name="parameters" element="trt:GetAudioOutputConfigurationsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetAudioDecoderConfigurationsRequest">
		<wsdl:part name="parameters" element="trt:GetAudioDecoderConfigurations"/>
	</wsdl:message>
	<wsdl:message name="GetAudioDecoderConfigurationsResponse">
		<wsdl:part name="parameters" element="trt:GetAudioDecoderConfigurationsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetVideoSourceConfigurationRequest">
		<wsdl:part name="parameters" element="trt:GetVideoSourceConfiguration"/>
	</wsdl:message>
	<wsdl:message name="GetVideoSourceConfigurationResponse">
		<wsdl:part name="parameters" element="trt:GetVideoSourceConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="GetVideoEncoderConfigurationRequest">
		<wsdl:part name="parameters" element="trt:GetVideoEncoderConfiguration"/>
	</wsdl:message>
	<wsdl:message name="GetVideoEncoderConfigurationResponse">
		<wsdl:part name="parameters" element="trt:GetVideoEncoderConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="GetAudioSourceConfigurationRequest">
		<wsdl:part name="parameters" element="trt:GetAudioSourceConfiguration"/>
	</wsdl:message>
	<wsdl:message name="GetAudioSourceConfigurationResponse">
		<wsdl:part name="parameters" element="trt:GetAudioSourceConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="GetAudioEncoderConfigurationRequest">
		<wsdl:part name="parameters" element="trt:GetAudioEncoderConfiguration"/>
	</wsdl:message>
	<wsdl:message name="GetAudioEncoderConfigurationResponse">
		<wsdl:part name="parameters" element="trt:GetAudioEncoderConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="GetVideoAnalyticsConfigurationRequest">
		<wsdl:part name="parameters" element="trt:GetVideoAnalyticsConfiguration"/>
	</wsdl:message>
	<wsdl:message name="GetVideoAnalyticsConfigurationResponse">
		<wsdl:part name="parameters" element="trt:GetVideoAnalyticsConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="GetMetadataConfigurationRequest">
		<wsdl:part name="parameters" element="trt:GetMetadataConfiguration"/>
	</wsdl:message>
	<wsdl:message name="GetMetadataConfigurationResponse">
		<wsdl:part name="parameters" element="trt:GetMetadataConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="GetAudioOutputConfigurationRequest">
		<wsdl:part name="parameters" element="trt:GetAudioOutputConfiguration"/>
	</wsdl:message>
	<wsdl:message name="GetAudioOutputConfigurationResponse">
		<wsdl:part name="parameters" element="trt:GetAudioOutputConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="GetAudioDecoderConfigurationRequest">
		<wsdl:part name="parameters" element="trt:GetAudioDecoderConfiguration"/>
	</wsdl:message>
	<wsdl:message name="GetAudioDecoderConfigurationResponse">
		<wsdl:part name="parameters" element="trt:GetAudioDecoderConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="GetCompatibleVideoEncoderConfigurationsRequest">
		<wsdl:part name="parameters" element="trt:GetCompatibleVideoEncoderConfigurations"/>
	</wsdl:message>
	<wsdl:message name="GetCompatibleVideoEncoderConfigurationsResponse">
		<wsdl:part name="parameters" element="trt:GetCompatibleVideoEncoderConfigurationsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetCompatibleVideoSourceConfigurationsRequest">
		<wsdl:part name="parameters" element="trt:GetCompatibleVideoSourceConfigurations"/>
	</wsdl:message>
	<wsdl:message name="GetCompatibleVideoSourceConfigurationsResponse">
		<wsdl:part name="parameters" element="trt:GetCompatibleVideoSourceConfigurationsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetCompatibleAudioEncoderConfigurationsRequest">
		<wsdl:part name="parameters" element="trt:GetCompatibleAudioEncoderConfigurations"/>
	</wsdl:message>
	<wsdl:message name="GetCompatibleAudioEncoderConfigurationsResponse">
		<wsdl:part name="parameters" element="trt:GetCompatibleAudioEncoderConfigurationsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetCompatibleAudioSourceConfigurationsRequest">
		<wsdl:part name="parameters" element="trt:GetCompatibleAudioSourceConfigurations"/>
	</wsdl:message>
	<wsdl:message name="GetCompatibleAudioSourceConfigurationsResponse">
		<wsdl:part name="parameters" element="trt:GetCompatibleAudioSourceConfigurationsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetCompatibleVideoAnalyticsConfigurationsRequest">
		<wsdl:part name="parameters" element="trt:GetCompatibleVideoAnalyticsConfigurations"/>
	</wsdl:message>
	<wsdl:message name="GetCompatibleVideoAnalyticsConfigurationsResponse">
		<wsdl:part name="parameters" element="trt:GetCompatibleVideoAnalyticsConfigurationsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetCompatibleMetadataConfigurationsRequest">
		<wsdl:part name="parameters" element="trt:GetCompatibleMetadataConfigurations"/>
	</wsdl:message>
	<wsdl:message name="GetCompatibleMetadataConfigurationsResponse">
		<wsdl:part name="parameters" element="trt:GetCompatibleMetadataConfigurationsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetCompatibleAudioOutputConfigurationsRequest">
		<wsdl:part name="parameters" element="trt:GetCompatibleAudioOutputConfigurations"/>
	</wsdl:message>
	<wsdl:message name="GetCompatibleAudioOutputConfigurationsResponse">
		<wsdl:part name="parameters" element="trt:GetCompatibleAudioOutputConfigurationsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetCompatibleAudioDecoderConfigurationsRequest">
		<wsdl:part name="parameters" element="trt:GetCompatibleAudioDecoderConfigurations"/>
	</wsdl:message>
	<wsdl:message name="GetCompatibleAudioDecoderConfigurationsResponse">
		<wsdl:part name="parameters" element="trt:GetCompatibleAudioDecoderConfigurationsResponse"/>
	</wsdl:message>
	<wsdl:message name="SetVideoSourceConfigurationRequest">
		<wsdl:part name="parameters" element="trt:SetVideoSourceConfiguration"/>
	</wsdl:message>
	<wsdl:message name="SetVideoSourceConfigurationResponse">
		<wsdl:part name="parameters" element="trt:SetVideoSourceConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="SetVideoEncoderConfigurationRequest">
		<wsdl:part name="parameters" element="trt:SetVideoEncoderConfiguration"/>
	</wsdl:message>
	<wsdl:message name="SetVideoEncoderConfigurationResponse">
		<wsdl:part name="parameters" element="trt:SetVideoEncoderConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="SetAudioSourceConfigurationRequest">
		<wsdl:part name="parameters" element="trt:SetAudioSourceConfiguration"/>
	</wsdl:message>
	<wsdl:message name="SetAudioSourceConfigurationResponse">
		<wsdl:part name="parameters" element="trt:SetAudioSourceConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="SetAudioEncoderConfigurationRequest">
		<wsdl:part name="parameters" element="trt:SetAudioEncoderConfiguration"/>
	</wsdl:message>
	<wsdl:message name="SetAudioEncoderConfigurationResponse">
		<wsdl:part name="parameters" element="trt:SetAudioEncoderConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="SetVideoAnalyticsConfigurationRequest">
		<wsdl:part name="parameters" element="trt:SetVideoAnalyticsConfiguration"/>
	</wsdl:message>
	<wsdl:message name="SetVideoAnalyticsConfigurationResponse">
		<wsdl:part name="parameters" element="trt:SetVideoAnalyticsConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="SetMetadataConfigurationRequest">
		<wsdl:part name="parameters" element="trt:SetMetadataConfiguration"/>
	</wsdl:message>
	<wsdl:message name="SetMetadataConfigurationResponse">
		<wsdl:part name="parameters" element="trt:SetMetadataConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="SetAudioOutputConfigurationRequest">
		<wsdl:part name="parameters" element="trt:SetAudioOutputConfiguration"/>
	</wsdl:message>
	<wsdl:message name="SetAudioOutputConfigurationResponse">
		<wsdl:part name="parameters" element="trt:SetAudioOutputConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="SetAudioDecoderConfigurationRequest">
		<wsdl:part name="parameters" element="trt:SetAudioDecoderConfiguration"/>
	</wsdl:message>
	<wsdl:message name="SetAudioDecoderConfigurationResponse">
		<wsdl:part name="parameters" element="trt:SetAudioDecoderConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="GetVideoSourceConfigurationOptionsRequest">
		<wsdl:part name="parameters" element="trt:GetVideoSourceConfigurationOptions"/>
	</wsdl:message>
	<wsdl:message name="GetVideoSourceConfigurationOptionsResponse">
		<wsdl:part name="parameters" element="trt:GetVideoSourceConfigurationOptionsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetVideoEncoderConfigurationOptionsRequest">
		<wsdl:part name="parameters" element="trt:GetVideoEncoderConfigurationOptions"/>
	</wsdl:message>
	<wsdl:message name="GetVideoEncoderConfigurationOptionsResponse">
		<wsdl:part name="parameters" element="trt:GetVideoEncoderConfigurationOptionsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetAudioSourceConfigurationOptionsRequest">
		<wsdl:part name="parameters" element="trt:GetAudioSourceConfigurationOptions"/>
	</wsdl:message>
	<wsdl:message name="GetAudioSourceConfigurationOptionsResponse">
		<wsdl:part name="parameters" element="trt:GetAudioSourceConfigurationOptionsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetAudioEncoderConfigurationOptionsRequest">
		<wsdl:part name="parameters" element="trt:GetAudioEncoderConfigurationOptions"/>
	</wsdl:message>
	<wsdl:message name="GetAudioEncoderConfigurationOptionsResponse">
		<wsdl:part name="parameters" element="trt:GetAudioEncoderConfigurationOptionsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetMetadataConfigurationOptionsRequest">
		<wsdl:part name="parameters" element="trt:GetMetadataConfigurationOptions"/>
	</wsdl:message>
	<wsdl:message name="GetMetadataConfigurationOptionsResponse">
		<wsdl:part name="parameters" element="trt:GetMetadataConfigurationOptionsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetAudioOutputConfigurationOptionsRequest">
		<wsdl:part name="parameters" element="trt:GetAudioOutputConfigurationOptions"/>
	</wsdl:message>
	<wsdl:message name="GetAudioOutputConfigurationOptionsResponse">
		<wsdl:part name="parameters" element="trt:GetAudioOutputConfigurationOptionsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetAudioDecoderConfigurationOptionsRequest">
		<wsdl:part name="parameters" element="trt:GetAudioDecoderConfigurationOptions"/>
	</wsdl:message>
	<wsdl:message name="GetAudioDecoderConfigurationOptionsResponse">
		<wsdl:part name="parameters" element="trt:GetAudioDecoderConfigurationOptionsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetGuaranteedNumberOfVideoEncoderInstancesRequest">
		<wsdl:part name="parameters" element="trt:GetGuaranteedNumberOfVideoEncoderInstances"/>
	</wsdl:message>
	<wsdl:message name="GetGuaranteedNumberOfVideoEncoderInstancesResponse">
		<wsdl:part name="parameters" element="trt:GetGuaranteedNumberOfVideoEncoderInstancesResponse"/>
	</wsdl:message>
	<wsdl:message name="GetStreamUriRequest">
		<wsdl:part name="parameters" element="trt:GetStreamUri"/>
	</wsdl:message>
	<wsdl:message name="GetStreamUriResponse">
		<wsdl:part name="parameters" element="trt:GetStreamUriResponse"/>
	</wsdl:message>
	<wsdl:message name="StartMulticastStreamingRequest">
		<wsdl:part name="parameters" element="trt:StartMulticastStreaming"/>
	</wsdl:message>
	<wsdl:message name="StartMulticastStreamingResponse">
		<wsdl:part name="parameters" element="trt:StartMulticastStreamingResponse"/>
	</wsdl:message>
	<wsdl:message name="StopMulticastStreamingRequest">
		<wsdl:part name="parameters" element="trt:StopMulticastStreaming"/>
	</wsdl:message>
	<wsdl:message name="StopMulticastStreamingResponse">
		<wsdl:part name="parameters" element="trt:StopMulticastStreamingResponse"/>
	</wsdl:message>
	<wsdl:message name="SetSynchronizationPointRequest">
		<wsdl:part name="parameters" element="trt:SetSynchronizationPoint"/>
	</wsdl:message>
	<wsdl:message name="SetSynchronizationPointResponse">
		<wsdl:part name="parameters" element="trt:SetSynchronizationPointResponse"/>
	</wsdl:message>
	<wsdl:message name="GetSnapshotUriRequest">
		<wsdl:part name="parameters" element="trt:GetSnapshotUri"/>
	</wsdl:message>
	<wsdl:message name="GetSnapshotUriResponse">
		<wsdl:part name="parameters" element="trt:GetSnapshotUriResponse"/>
	</wsdl:message>
	<wsdl:message name="GetVideoSourceModesRequest">
		<wsdl:part name="parameters" element="trt:GetVideoSourceModes"/>
	</wsdl:message>
	<wsdl:message name="GetVideoSourceModesResponse">
		<wsdl:part name="parameters" element="trt:GetVideoSourceModesResponse"/>
	</wsdl:message>
	<wsdl:message name="SetVideoSourceModeRequest">
		<wsdl:part name="parameters" element="trt:SetVideoSourceMode"/>
	</wsdl:message>
	<wsdl:message name="SetVideoSourceModeResponse">
		<wsdl:part name="parameters" element="trt:SetVideoSourceModeResponse"/>
	</wsdl:message>
	<wsdl:message name="GetOSDsRequest">
		<wsdl:part name="parameters" element="trt:GetOSDs"/>
	</wsdl:message>
	<wsdl:message name="GetOSDsResponse">
		<wsdl:part name="parameters" element="trt:GetOSDsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetOSDRequest">
		<wsdl:part name="parameters" element="trt:GetOSD"/>
	</wsdl:message>
	<wsdl:message name="GetOSDResponse">
		<wsdl:part name="parameters" element="trt:GetOSDResponse"/>
	</wsdl:message>
	<wsdl:message name="GetOSDOptionsRequest">
		<wsdl:part name="parameters" element="trt:GetOSDOptions"/>
	</wsdl:message>
	<wsdl:message name="GetOSDOptionsResponse">
		<wsdl:part name="parameters" element="trt:GetOSDOptionsResponse"/>
	</wsdl:message>
	<wsdl:message name="SetOSDRequest">
		<wsdl:part name="parameters" element="trt:SetOSD"/>
	</wsdl:message>
	<wsdl:message name="SetOSDResponse">
		<wsdl:part name="parameters" element="trt:SetOSDResponse"/>
	</wsdl:message>
	<wsdl:message name="CreateOSDRequest">
		<wsdl:part name="parameters" element="trt:CreateOSD"/>
	</wsdl:message>
	<wsdl:message name="CreateOSDResponse">
		<wsdl:part name="parameters" element="trt:CreateOSDResponse"/>
	</wsdl:message>
	<wsdl:message name="DeleteOSDRequest">
		<wsdl:part name="parameters" element="trt:DeleteOSD"/>
	</wsdl:message>
	<wsdl:message name="DeleteOSDResponse">
		<wsdl:part name="parameters" element="trt:DeleteOSDResponse"/>
	</wsdl:message>
	<wsdl:portType name="Media">
		<!--===============================-->
		<!--===============================-->
		<wsdl:operation name="GetServiceCapabilities">
			<wsdl:documentation>Returns the capabilities of the media service. The result is returned in a typed answer.</wsdl:documentation>
			<wsdl:input message="trt:GetServiceCapabilitiesRequest"/>
			<wsdl:output message="trt:GetServiceCapabilitiesResponse"/>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetVideoSources">
			<wsdl:documentation>This command lists all available physical video inputs of the device.</wsdl:documentation>
			<wsdl:input message="trt:GetVideoSourcesRequest"/>
			<wsdl:output message="trt:GetVideoSourcesResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetAudioSources">
			<wsdl:documentation>This command lists all available physical audio inputs of the device.</wsdl:documentation>
			<wsdl:input message="trt:GetAudioSourcesRequest"/>
			<wsdl:output message="trt:GetAudioSourcesResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetAudioOutputs">
			<wsdl:documentation>This command lists all available physical audio outputs of the device.</wsdl:documentation>
			<wsdl:input message="trt:GetAudioOutputsRequest"/>
			<wsdl:output message="trt:GetAudioOutputsResponse"/>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="CreateProfile">
			<wsdl:documentation>This operation creates a new empty media profile. The media profile shall be created in the
device and shall be persistent (remain after reboot). A created profile shall be deletable and a device shall set the “fixed” attribute to false in the
returned Profile.</wsdl:documentation>
			<wsdl:input message="trt:CreateProfileRequest"/>
			<wsdl:output message="trt:CreateProfileResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetProfile">
			<wsdl:documentation>If the profile token is already known, a profile can be fetched through the GetProfile command.</wsdl:documentation>
			<wsdl:input message="trt:GetProfileRequest"/>
			<wsdl:output message="trt:GetProfileResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetProfiles">
			<wsdl:documentation>Any endpoint can ask for the existing media profiles of a device using the GetProfiles
command. Pre-configured or dynamically configured profiles can be retrieved using this
command. This command lists all configured profiles in a device. The client does not need to
know the media profile in order to use the command.</wsdl:documentation>
			<wsdl:input message="trt:GetProfilesRequest"/>
			<wsdl:output message="trt:GetProfilesResponse"/>
		</wsdl:operation>
		<wsdl:operation name="AddVideoEncoderConfiguration">
			<wsdl:documentation>This operation adds a VideoEncoderConfiguration to an existing media profile. If a
configuration exists in the media profile, it will be replaced. The change shall be persistent. A device shall
support adding a compatible VideoEncoderConfiguration to a Profile containing a VideoSourceConfiguration and shall
support streaming video data of such a profile.
			</wsdl:documentation>
			<wsdl:input message="trt:AddVideoEncoderConfigurationRequest"/>
			<wsdl:output message="trt:AddVideoEncoderConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="RemoveVideoEncoderConfiguration">
			<wsdl:documentation>This operation removes a VideoEncoderConfiguration from an existing media profile. If the
media profile does not contain a VideoEncoderConfiguration, the operation has no effect. The removal shall be persistent.</wsdl:documentation>
			<wsdl:input message="trt:RemoveVideoEncoderConfigurationRequest"/>
			<wsdl:output message="trt:RemoveVideoEncoderConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="AddVideoSourceConfiguration">
			<wsdl:documentation>This operation adds a VideoSourceConfiguration to an existing media profile. If such a
configuration exists in the media profile, it will be replaced. The change shall be persistent.</wsdl:documentation>
			<wsdl:input message="trt:AddVideoSourceConfigurationRequest"/>
			<wsdl:output message="trt:AddVideoSourceConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="RemoveVideoSourceConfiguration">
			<wsdl:documentation>This operation removes a VideoSourceConfiguration from an existing media profile. If the
media profile does not contain a VideoSourceConfiguration, the operation has no effect. The removal shall be persistent. Video source configurations should only be removed after removing a
VideoEncoderConfiguration from the media profile.</wsdl:documentation>
			<wsdl:input message="trt:RemoveVideoSourceConfigurationRequest"/>
			<wsdl:output message="trt:RemoveVideoSourceConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="AddAudioEncoderConfiguration">
			<wsdl:documentation>This operation adds an AudioEncoderConfiguration to an existing media profile. If a 
configuration exists in the media profile, it will be replaced. The change shall be persistent. A device shall
support adding a compatible AudioEncoderConfiguration to a profile containing an AudioSourceConfiguration and shall
support streaming audio data of such a profile.
			</wsdl:documentation>
			<wsdl:input message="trt:AddAudioEncoderConfigurationRequest"/>
			<wsdl:output message="trt:AddAudioEncoderConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="RemoveAudioEncoderConfiguration">
			<wsdl:documentation>This operation removes an AudioEncoderConfiguration from an existing media profile. If the
media profile does not contain an AudioEncoderConfiguration, the operation has no effect.
The removal shall be persistent.</wsdl:documentation>
			<wsdl:input message="trt:RemoveAudioEncoderConfigurationRequest"/>
			<wsdl:output message="trt:RemoveAudioEncoderConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="AddAudioSourceConfiguration">
			<wsdl:documentation>This operation adds an AudioSourceConfiguration to an existing media profile. If a
configuration exists in the media profile, it will be replaced. The change shall be persistent.</wsdl:documentation>
			<wsdl:input message="trt:AddAudioSourceConfigurationRequest"/>
			<wsdl:output message="trt:AddAudioSourceConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="RemoveAudioSourceConfiguration">
			<wsdl:documentation>This operation removes an AudioSourceConfiguration from an existing media profile. If the
media profile does not contain an AudioSourceConfiguration, the operation has no effect. The
removal shall be persistent. Audio source configurations should only be removed after removing an
AudioEncoderConfiguration from the media profile.</wsdl:documentation>
			<wsdl:input message="trt:RemoveAudioSourceConfigurationRequest"/>
			<wsdl:output message="trt:RemoveAudioSourceConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="AddPTZConfiguration">
			<wsdl:documentation>This operation adds a PTZConfiguration to an existing media profile. If a configuration exists
in the media profile, it will be replaced. The change shall be persistent. Adding a PTZConfiguration to a media profile means that streams using that media profile can
contain PTZ status (in the metadata), and that the media profile can be used for controlling
PTZ movement.</wsdl:documentation>
			<wsdl:input message="trt:AddPTZConfigurationRequest"/>
			<wsdl:output message="trt:AddPTZConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="RemovePTZConfiguration">
			<wsdl:documentation>This operation removes a PTZConfiguration from an existing media profile. If the media profile
does not contain a PTZConfiguration, the operation has no effect. The removal shall be persistent.</wsdl:documentation>
			<wsdl:input message="trt:RemovePTZConfigurationRequest"/>
			<wsdl:output message="trt:RemovePTZConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="AddVideoAnalyticsConfiguration">
			<wsdl:documentation>This operation adds a VideoAnalytics configuration to an existing media profile. If a
configuration exists in the media profile, it will be replaced. The change shall be persistent. Adding a VideoAnalyticsConfiguration to a media profile means that streams using that media
profile can contain video analytics data (in the metadata) as defined by the submitted configuration reference. A profile containing only a video analytics configuration but no video source configuration is incomplete. Therefore, a client should first add a video source configuration to a profile before adding a video analytics configuration. The device can deny adding of a video analytics
configuration before a video source configuration.</wsdl:documentation>
			<wsdl:input message="trt:AddVideoAnalyticsConfigurationRequest"/>
			<wsdl:output message="trt:AddVideoAnalyticsConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="RemoveVideoAnalyticsConfiguration">
			<wsdl:documentation>This operation removes a VideoAnalyticsConfiguration from an existing media profile. If the media profile does not contain a VideoAnalyticsConfiguration, the operation has no effect.
The removal shall be persistent.</wsdl:documentation>
			<wsdl:input message="trt:RemoveVideoAnalyticsConfigurationRequest"/>
			<wsdl:output message="trt:RemoveVideoAnalyticsConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="AddMetadataConfiguration">
			<wsdl:documentation>This operation adds a Metadata configuration to an existing media profile. If a configuration exists in the media profile, it will be replaced. The change shall be persistent. Adding a MetadataConfiguration to a Profile means that streams using that profile contain metadata. Metadata can consist of events, PTZ status, and/or video analytics data.</wsdl:documentation>
			<wsdl:input message="trt:AddMetadataConfigurationRequest"/>
			<wsdl:output message="trt:AddMetadataConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="RemoveMetadataConfiguration">
			<wsdl:documentation>This operation removes a MetadataConfiguration from an existing media profile. If the media profile does not contain a MetadataConfiguration, the operation has no effect. The removal shall be persistent.</wsdl:documentation>
			<wsdl:input message="trt:RemoveMetadataConfigurationRequest"/>
			<wsdl:output message="trt:RemoveMetadataConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="AddAudioOutputConfiguration">
			<wsdl:documentation>This operation adds an AudioOutputConfiguration to an existing media profile. If a configuration exists in the media profile, it will be replaced. The change shall be persistent.</wsdl:documentation>
			<wsdl:input message="trt:AddAudioOutputConfigurationRequest"/>
			<wsdl:output message="trt:AddAudioOutputConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="RemoveAudioOutputConfiguration">
			<wsdl:documentation>This operation removes an AudioOutputConfiguration from an existing media profile. If the media profile does not contain an AudioOutputConfiguration, the operation has no effect. The removal shall be persistent.</wsdl:documentation>
			<wsdl:input message="trt:RemoveAudioOutputConfigurationRequest"/>
			<wsdl:output message="trt:RemoveAudioOutputConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="AddAudioDecoderConfiguration">
			<wsdl:documentation>This operation adds an AudioDecoderConfiguration to an existing media profile. If a configuration exists in the media profile, it shall be replaced. The change shall be persistent.</wsdl:documentation>
			<wsdl:input message="trt:AddAudioDecoderConfigurationRequest"/>
			<wsdl:output message="trt:AddAudioDecoderConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="RemoveAudioDecoderConfiguration">
			<wsdl:documentation>This operation removes an AudioDecoderConfiguration from an existing media profile. If the media profile does not contain an AudioDecoderConfiguration, the operation has no effect. The removal shall be persistent.</wsdl:documentation>
			<wsdl:input message="trt:RemoveAudioDecoderConfigurationRequest"/>
			<wsdl:output message="trt:RemoveAudioDecoderConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="DeleteProfile">
			<wsdl:documentation>This operation deletes a profile. This change shall always be persistent. Deletion of a profile is only possible for non-fixed profiles</wsdl:documentation>
			<wsdl:input message="trt:DeleteProfileRequest"/>
			<wsdl:output message="trt:DeleteProfileResponse"/>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetVideoSourceConfigurations">
			<wsdl:documentation>This operation lists all existing video source configurations for a device. The client need not know anything about the video source configurations in order to use the command.</wsdl:documentation>
			<wsdl:input message="trt:GetVideoSourceConfigurationsRequest"/>
			<wsdl:output message="trt:GetVideoSourceConfigurationsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetVideoEncoderConfigurations">
			<wsdl:documentation>This operation lists all existing video encoder configurations of a device. This command lists all configured video encoder configurations in a device. The client need not know anything apriori about the video encoder configurations in order to use the command.</wsdl:documentation>
			<wsdl:input message="trt:GetVideoEncoderConfigurationsRequest"/>
			<wsdl:output message="trt:GetVideoEncoderConfigurationsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetAudioSourceConfigurations">
			<wsdl:documentation>This operation lists all existing audio source configurations of a device. This command lists all audio source configurations in a device. The client need not know anything apriori about the audio source configurations in order to use the command.</wsdl:documentation>
			<wsdl:input message="trt:GetAudioSourceConfigurationsRequest"/>
			<wsdl:output message="trt:GetAudioSourceConfigurationsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetAudioEncoderConfigurations">
			<wsdl:documentation>This operation lists all existing device audio encoder configurations. The client need not know anything apriori about the audio encoder configurations in order to use the command.</wsdl:documentation>
			<wsdl:input message="trt:GetAudioEncoderConfigurationsRequest"/>
			<wsdl:output message="trt:GetAudioEncoderConfigurationsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetVideoAnalyticsConfigurations">
			<wsdl:documentation>This operation lists all video analytics configurations of a device. This command lists all configured video analytics in a device. The client need not know anything apriori about the video analytics in order to use the command.</wsdl:documentation>
			<wsdl:input message="trt:GetVideoAnalyticsConfigurationsRequest"/>
			<wsdl:output message="trt:GetVideoAnalyticsConfigurationsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetMetadataConfigurations">
			<wsdl:documentation>This operation lists all existing metadata configurations. The client need not know anything apriori about the metadata in order to use the command.</wsdl:documentation>
			<wsdl:input message="trt:GetMetadataConfigurationsRequest"/>
			<wsdl:output message="trt:GetMetadataConfigurationsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetAudioOutputConfigurations">
			<wsdl:documentation>This command lists all existing AudioOutputConfigurations of a device. The NVC need not know anything apriori about the audio configurations to use this command.</wsdl:documentation>
			<wsdl:input message="trt:GetAudioOutputConfigurationsRequest"/>
			<wsdl:output message="trt:GetAudioOutputConfigurationsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetAudioDecoderConfigurations">
			<wsdl:documentation>This command lists all existing AudioDecoderConfigurations of a device. The NVC need not know anything apriori about the audio decoder configurations in order to
use this command.</wsdl:documentation>
			<wsdl:input message="trt:GetAudioDecoderConfigurationsRequest"/>
			<wsdl:output message="trt:GetAudioDecoderConfigurationsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetVideoSourceConfiguration">
			<wsdl:documentation>If the video source configuration token is already known, the video source configuration can be fetched through the GetVideoSourceConfiguration command.</wsdl:documentation>
			<wsdl:input message="trt:GetVideoSourceConfigurationRequest"/>
			<wsdl:output message="trt:GetVideoSourceConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetVideoEncoderConfiguration">
			<wsdl:documentation>If the video encoder configuration token is already known, the encoder configuration can be fetched through the GetVideoEncoderConfiguration command.</wsdl:documentation>
			<wsdl:input message="trt:GetVideoEncoderConfigurationRequest"/>
			<wsdl:output message="trt:GetVideoEncoderConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetAudioSourceConfiguration">
			<wsdl:documentation>The GetAudioSourceConfiguration command fetches the audio source configurations if the audio source configuration token is already known. An</wsdl:documentation>
			<wsdl:input message="trt:GetAudioSourceConfigurationRequest"/>
			<wsdl:output message="trt:GetAudioSourceConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetAudioEncoderConfiguration">
			<wsdl:documentation>The GetAudioEncoderConfiguration command fetches the encoder configuration if the audio encoder configuration token is known.</wsdl:documentation>
			<wsdl:input message="trt:GetAudioEncoderConfigurationRequest"/>
			<wsdl:output message="trt:GetAudioEncoderConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetVideoAnalyticsConfiguration">
			<wsdl:documentation>The GetVideoAnalyticsConfiguration command fetches the video analytics configuration if the video analytics token is known.</wsdl:documentation>
			<wsdl:input message="trt:GetVideoAnalyticsConfigurationRequest"/>
			<wsdl:output message="trt:GetVideoAnalyticsConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetMetadataConfiguration">
			<wsdl:documentation>The GetMetadataConfiguration command fetches the metadata configuration if the metadata token is known.</wsdl:documentation>
			<wsdl:input message="trt:GetMetadataConfigurationRequest"/>
			<wsdl:output message="trt:GetMetadataConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetAudioOutputConfiguration">
			<wsdl:documentation>If the audio output configuration token is already known, the output configuration can be fetched through the GetAudioOutputConfiguration command.</wsdl:documentation>
			<wsdl:input message="trt:GetAudioOutputConfigurationRequest"/>
			<wsdl:output message="trt:GetAudioOutputConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetAudioDecoderConfiguration">
			<wsdl:documentation>If the audio decoder configuration token is already known, the decoder configuration can be fetched through the GetAudioDecoderConfiguration command.</wsdl:documentation>
			<wsdl:input message="trt:GetAudioDecoderConfigurationRequest"/>
			<wsdl:output message="trt:GetAudioDecoderConfigurationResponse"/>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetCompatibleVideoEncoderConfigurations">
			<wsdl:documentation>This operation lists all the video encoder configurations of the device that are compatible with a certain media profile. Each of the returned configurations shall be a valid input parameter for the AddVideoEncoderConfiguration command on the media profile. The result will vary depending on the capabilities, configurations and settings in the device.</wsdl:documentation>
			<wsdl:input message="trt:GetCompatibleVideoEncoderConfigurationsRequest"/>
			<wsdl:output message="trt:GetCompatibleVideoEncoderConfigurationsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetCompatibleVideoSourceConfigurations">
			<wsdl:documentation>This operation requests all the video source configurations of the device that are compatible
with a certain media profile. Each of the returned configurations shall be a valid input
parameter for the AddVideoSourceConfiguration command on the media profile. The result
will vary depending on the capabilities, configurations and settings in the device.</wsdl:documentation>
			<wsdl:input message="trt:GetCompatibleVideoSourceConfigurationsRequest"/>
			<wsdl:output message="trt:GetCompatibleVideoSourceConfigurationsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetCompatibleAudioEncoderConfigurations">
			<wsdl:documentation>This operation requests all audio encoder configurations of a device that are compatible with a certain media profile. Each of the returned configurations shall be a valid input parameter for the AddAudioSourceConfiguration command on the media profile. The result varies depending on the capabilities, configurations and settings in the device.</wsdl:documentation>
			<wsdl:input message="trt:GetCompatibleAudioEncoderConfigurationsRequest"/>
			<wsdl:output message="trt:GetCompatibleAudioEncoderConfigurationsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetCompatibleAudioSourceConfigurations">
			<wsdl:documentation>This operation requests all audio source configurations of the device that are compatible with a certain media profile. Each of the returned configurations shall be a valid input parameter for the AddAudioEncoderConfiguration command on the media profile. The result varies depending on the capabilities, configurations and settings in the device.</wsdl:documentation>
			<wsdl:input message="trt:GetCompatibleAudioSourceConfigurationsRequest"/>
			<wsdl:output message="trt:GetCompatibleAudioSourceConfigurationsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetCompatibleVideoAnalyticsConfigurations">
			<wsdl:documentation>This operation requests all video analytic configurations of the device that are compatible with a certain media profile. Each of the returned configurations shall be a valid input parameter for the AddVideoAnalyticsConfiguration command on the media profile. The result varies depending on the capabilities, configurations and settings in the device.</wsdl:documentation>
			<wsdl:input message="trt:GetCompatibleVideoAnalyticsConfigurationsRequest"/>
			<wsdl:output message="trt:GetCompatibleVideoAnalyticsConfigurationsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetCompatibleMetadataConfigurations">
			<wsdl:documentation>This operation requests all the metadata configurations of the device that are compatible with a certain media profile. Each of the returned configurations shall be a valid input parameter for the AddMetadataConfiguration command on the media profile. The result varies depending on the capabilities, configurations and settings in the device.</wsdl:documentation>
			<wsdl:input message="trt:GetCompatibleMetadataConfigurationsRequest"/>
			<wsdl:output message="trt:GetCompatibleMetadataConfigurationsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetCompatibleAudioOutputConfigurations">
			<wsdl:documentation>This command lists all audio output configurations of a device that are compatible with a certain media profile. Each returned configuration shall be a valid input for the 
AddAudioOutputConfiguration command.</wsdl:documentation>
			<wsdl:input message="trt:GetCompatibleAudioOutputConfigurationsRequest"/>
			<wsdl:output message="trt:GetCompatibleAudioOutputConfigurationsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetCompatibleAudioDecoderConfigurations">
			<wsdl:documentation>This operation lists all the audio decoder configurations of the device that are compatible with a certain media profile. Each of the returned configurations shall be a valid input parameter for the AddAudioDecoderConfiguration command on the media profile.</wsdl:documentation>
			<wsdl:input message="trt:GetCompatibleAudioDecoderConfigurationsRequest"/>
			<wsdl:output message="trt:GetCompatibleAudioDecoderConfigurationsResponse"/>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="SetVideoSourceConfiguration">
			<wsdl:documentation>This operation modifies a video source configuration. The ForcePersistence flag indicates if the changes shall remain after reboot of the device. Running streams using this configuration may be immediately updated according to the new settings. The changes are not guaranteed to take effect unless the client requests a new stream URI and restarts any affected stream. NVC methods for changing a running stream are out of scope for this specification.</wsdl:documentation>
			<wsdl:input message="trt:SetVideoSourceConfigurationRequest"/>
			<wsdl:output message="trt:SetVideoSourceConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetVideoEncoderConfiguration">
			<wsdl:documentation>This operation modifies a video encoder configuration. The ForcePersistence flag indicates if the changes shall remain after reboot of the device. Changes in the Multicast settings shall always be persistent. Running streams using this configuration may be immediately updated according to the new settings. The changes are not guaranteed to take effect unless the client requests a new stream URI and restarts any affected stream. NVC methods for changing a running stream are out of scope for this specification. <br/>SessionTimeout is provided as a hint for keeping rtsp session by a device. If necessary the device may adapt parameter values for SessionTimeout elements without returning an error. For the time between keep alive calls the client shall adhere to the timeout value signaled via RTSP.</wsdl:documentation>
			<wsdl:input message="trt:SetVideoEncoderConfigurationRequest"/>
			<wsdl:output message="trt:SetVideoEncoderConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetAudioSourceConfiguration">
			<wsdl:documentation>This operation modifies an audio source configuration. The ForcePersistence flag indicates if
the changes shall remain after reboot of the device. Running streams using this configuration
may be immediately updated according to the new settings. The changes are not guaranteed
to take effect unless the client requests a new stream URI and restarts any affected stream
NVC methods for changing a running stream are out of scope for this specification.</wsdl:documentation>
			<wsdl:input message="trt:SetAudioSourceConfigurationRequest"/>
			<wsdl:output message="trt:SetAudioSourceConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetAudioEncoderConfiguration">
			<wsdl:documentation>This operation modifies an audio encoder configuration. The ForcePersistence flag indicates if
the changes shall remain after reboot of the device. Running streams using this configuration may be immediately updated
according to the new settings. The changes are not guaranteed to take effect unless the client
requests a new stream URI and restarts any affected streams. NVC methods for changing a
running stream are out of scope for this specification.</wsdl:documentation>
			<wsdl:input message="trt:SetAudioEncoderConfigurationRequest"/>
			<wsdl:output message="trt:SetAudioEncoderConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetVideoAnalyticsConfiguration">
			<wsdl:documentation>A video analytics configuration is modified using this command. The ForcePersistence flag
indicates if the changes shall remain after reboot of the device or not. Running streams using
this configuration shall be immediately updated according to the new settings. Otherwise
inconsistencies can occur between the scene description processed by the rule engine and
the notifications produced by analytics engine and rule engine which reference the very same
video analytics configuration token.</wsdl:documentation>
			<wsdl:input message="trt:SetVideoAnalyticsConfigurationRequest"/>
			<wsdl:output message="trt:SetVideoAnalyticsConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetMetadataConfiguration">
			<wsdl:documentation>This operation modifies a metadata configuration. The ForcePersistence flag indicates if the
changes shall remain after reboot of the device. Changes in the Multicast settings shall
always be persistent. Running streams using this configuration may be updated immediately
according to the new settings. The changes are not guaranteed to take effect unless the client
requests a new stream URI and restarts any affected streams. NVC methods for changing a
running stream are out of scope for this specification.</wsdl:documentation>
			<wsdl:input message="trt:SetMetadataConfigurationRequest"/>
			<wsdl:output message="trt:SetMetadataConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetAudioOutputConfiguration">
			<wsdl:documentation>This operation modifies an audio output configuration. The ForcePersistence flag indicates if
the changes shall remain after reboot of the device.</wsdl:documentation>
			<wsdl:input message="trt:SetAudioOutputConfigurationRequest"/>
			<wsdl:output message="trt:SetAudioOutputConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetAudioDecoderConfiguration">
			<wsdl:documentation>This operation modifies an audio decoder configuration. The ForcePersistence flag indicates if
the changes shall remain after reboot of the device.</wsdl:documentation>
			<wsdl:input message="trt:SetAudioDecoderConfigurationRequest"/>
			<wsdl:output message="trt:SetAudioDecoderConfigurationResponse"/>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetVideoSourceConfigurationOptions">
			<wsdl:documentation>This operation returns the available options  (supported values and ranges for video source configuration parameters) when the video source parameters are
reconfigured If a video source configuration is specified, the options shall concern that
particular configuration. If a media profile is specified, the options shall be compatible with
that media profile.</wsdl:documentation>
			<wsdl:input message="trt:GetVideoSourceConfigurationOptionsRequest"/>
			<wsdl:output message="trt:GetVideoSourceConfigurationOptionsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetVideoEncoderConfigurationOptions">
			<wsdl:documentation>This operation returns the available options (supported values and ranges for video encoder 
				configuration parameters) when the video encoder parameters are reconfigured. <br/>
				For JPEG, MPEG4 and H264 extension elements have been defined that provide additional information. A device must provide the 
				XxxOption information for all encodings supported and should additionally provide the corresponding XxxOption2 information.<br/>
				This response contains the available video encoder configuration options. If a video encoder configuration is specified, 
				the options shall concern that particular configuration. If a media profile is specified, the options shall be 
				compatible with that media profile. If no tokens are specified, the options shall be considered generic for the device.
			</wsdl:documentation>
			<wsdl:input message="trt:GetVideoEncoderConfigurationOptionsRequest"/>
			<wsdl:output message="trt:GetVideoEncoderConfigurationOptionsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetAudioSourceConfigurationOptions">
			<wsdl:documentation>This operation returns the available options (supported values and ranges for audio source configuration parameters) when the audio source parameters are
reconfigured. If an audio source configuration is specified, the options shall concern that
particular configuration. If a media profile is specified, the options shall be compatible with
that media profile.</wsdl:documentation>
			<wsdl:input message="trt:GetAudioSourceConfigurationOptionsRequest"/>
			<wsdl:output message="trt:GetAudioSourceConfigurationOptionsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetAudioEncoderConfigurationOptions">
			<wsdl:documentation>This operation returns the available options  (supported values and ranges for audio encoder configuration parameters) when the audio encoder parameters are
reconfigured.</wsdl:documentation>
			<wsdl:input message="trt:GetAudioEncoderConfigurationOptionsRequest"/>
			<wsdl:output message="trt:GetAudioEncoderConfigurationOptionsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetMetadataConfigurationOptions">
			<wsdl:documentation>This operation returns the available options (supported values and ranges for metadata configuration parameters) for changing the metadata configuration.</wsdl:documentation>
			<wsdl:input message="trt:GetMetadataConfigurationOptionsRequest"/>
			<wsdl:output message="trt:GetMetadataConfigurationOptionsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetAudioOutputConfigurationOptions">
			<wsdl:documentation>This operation returns the available options (supported values and ranges for audio output configuration parameters) for configuring an audio output.</wsdl:documentation>
			<wsdl:input message="trt:GetAudioOutputConfigurationOptionsRequest"/>
			<wsdl:output message="trt:GetAudioOutputConfigurationOptionsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetAudioDecoderConfigurationOptions">
			<wsdl:documentation>This command list the audio decoding capabilities for a given profile and configuration of a
device.</wsdl:documentation>
			<wsdl:input message="trt:GetAudioDecoderConfigurationOptionsRequest"/>
			<wsdl:output message="trt:GetAudioDecoderConfigurationOptionsResponse"/>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetGuaranteedNumberOfVideoEncoderInstances">
			<wsdl:documentation>The GetGuaranteedNumberOfVideoEncoderInstances command can be used to request the
minimum number of guaranteed video encoder instances (applications) per Video Source
Configuration.</wsdl:documentation>
			<wsdl:input message="trt:GetGuaranteedNumberOfVideoEncoderInstancesRequest"/>
			<wsdl:output message="trt:GetGuaranteedNumberOfVideoEncoderInstancesResponse"/>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetStreamUri">
			<wsdl:documentation>This operation requests a URI that can be used to initiate a live media stream using RTSP as
the control protocol. The returned URI shall remain valid indefinitely even if the profile is
changed. The ValidUntilConnect, ValidUntilReboot and Timeout Parameter shall be set
accordingly (ValidUntilConnect=false, ValidUntilReboot=false, timeout=PT0S). <br/>
				The correct syntax for the StreamSetup element for these media stream setups defined in 5.1.1 of the streaming specification are as follows:
				<ol>
					<li>RTP unicast over UDP: StreamType = "RTP_unicast", TransportProtocol = "UDP"</li>
					<li>RTP over RTSP over HTTP over TCP: StreamType = "RTP_unicast", TransportProtocol = "HTTP"</li>
					<li>RTP over RTSP over TCP: StreamType = "RTP_unicast", TransportProtocol = "RTSP"</li>
				</ol>
				<br/>
If a multicast stream is requested at least one of VideoEncoderConfiguration, AudioEncoderConfiguration and MetadataConfiguration shall have a valid multicast setting.<br/>
For full compatibility with other ONVIF services a device should not generate Uris longer than
128 octets.</wsdl:documentation>
			<wsdl:input message="trt:GetStreamUriRequest"/>
			<wsdl:output message="trt:GetStreamUriResponse"/>
		</wsdl:operation>
		<wsdl:operation name="StartMulticastStreaming">
			<wsdl:documentation>This command starts multicast streaming using a specified media profile of a device.
Streaming continues until StopMulticastStreaming is called for the same Profile. The
streaming shall continue after a reboot of the device until a StopMulticastStreaming request is
received. The multicast address, port and TTL are configured in the
VideoEncoderConfiguration, AudioEncoderConfiguration and MetadataConfiguration
respectively.</wsdl:documentation>
			<wsdl:input message="trt:StartMulticastStreamingRequest"/>
			<wsdl:output message="trt:StartMulticastStreamingResponse"/>
		</wsdl:operation>
		<wsdl:operation name="StopMulticastStreaming">
			<wsdl:documentation>This command stop multicast streaming using a specified media profile of a device</wsdl:documentation>
			<wsdl:input message="trt:StopMulticastStreamingRequest"/>
			<wsdl:output message="trt:StopMulticastStreamingResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetSynchronizationPoint">
			<wsdl:documentation>Synchronization points allow clients to decode and correctly use all data after the
synchronization point.
For example, if a video stream is configured with a large I-frame distance and a client loses a
single packet, the client does not display video until the next I-frame is transmitted. In such
cases, the client can request a Synchronization Point which enforces the device to add an I-Frame as soon as possible. Clients can request Synchronization Points for profiles. The device
shall add synchronization points for all streams associated with this profile.
Similarly, a synchronization point is used to get an update on full PTZ or event status through
the metadata stream.
If a video stream is associated with the profile, an I-frame shall be added to this video stream.
If a PTZ metadata stream is associated to the profile,
the PTZ position shall be repeated within the metadata stream.</wsdl:documentation>
			<wsdl:input message="trt:SetSynchronizationPointRequest"/>
			<wsdl:output message="trt:SetSynchronizationPointResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetSnapshotUri">
			<wsdl:documentation>A client uses the GetSnapshotUri command to obtain a JPEG snapshot from the
device. The returned URI shall remain valid indefinitely even if the profile is changed. The
ValidUntilConnect, ValidUntilReboot and Timeout Parameter shall be set accordingly
(ValidUntilConnect=false, ValidUntilReboot=false, timeout=PT0S). The URI can be used for
acquiring a JPEG image through a HTTP GET operation. The image encoding will always be
JPEG regardless of the encoding setting in the media profile. The Jpeg settings
(like resolution or quality) may be taken from the profile if suitable. The provided
image will be updated automatically and independent from calls to GetSnapshotUri.</wsdl:documentation>
			<wsdl:input message="trt:GetSnapshotUriRequest"/>
			<wsdl:output message="trt:GetSnapshotUriResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetVideoSourceModes">
			<wsdl:documentation>A device returns the information for current video source mode and settable video source modes of specified video source. A device that indicates a capability of  VideoSourceModes shall support this command.</wsdl:documentation>
			<wsdl:input message="trt:GetVideoSourceModesRequest"/>
			<wsdl:output message="trt:GetVideoSourceModesResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetVideoSourceMode">
			<wsdl:documentation>SetVideoSourceMode changes the media profile structure relating to video source for the specified video source mode. A device that indicates a capability of VideoSourceModes shall support this command. The behavior after changing the mode is not defined in this specification.</wsdl:documentation>
			<wsdl:input message="trt:SetVideoSourceModeRequest"/>
			<wsdl:output message="trt:SetVideoSourceModeResponse"/>
		</wsdl:operation>
		<!--==============OSD Operation Begin=================-->
		<wsdl:operation name="GetOSDs">
			<wsdl:documentation>Get the OSDs.</wsdl:documentation>
			<wsdl:input message="trt:GetOSDsRequest"/>
			<wsdl:output message="trt:GetOSDsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetOSD">
			<wsdl:documentation>Get the OSD.</wsdl:documentation>
			<wsdl:input message="trt:GetOSDRequest"/>
			<wsdl:output message="trt:GetOSDResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetOSDOptions">
			<wsdl:documentation>Get the OSD Options.</wsdl:documentation>
			<wsdl:input message="trt:GetOSDOptionsRequest"/>
			<wsdl:output message="trt:GetOSDOptionsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetOSD">
			<wsdl:documentation>Set the OSD</wsdl:documentation>
			<wsdl:input message="trt:SetOSDRequest"/>
			<wsdl:output message="trt:SetOSDResponse"/>
		</wsdl:operation>
		<wsdl:operation name="CreateOSD">
			<wsdl:documentation>Create the OSD.</wsdl:documentation>
			<wsdl:input message="trt:CreateOSDRequest"/>
			<wsdl:output message="trt:CreateOSDResponse"/>
		</wsdl:operation>
		<wsdl:operation name="DeleteOSD">
			<wsdl:documentation>Delete the OSD.</wsdl:documentation>
			<wsdl:input message="trt:DeleteOSDRequest"/>
			<wsdl:output message="trt:DeleteOSDResponse"/>
		</wsdl:operation>
		<!--==============OSD Operation End=================-->
	</wsdl:portType>
	<wsdl:binding name="MediaBinding" type="trt:Media">
		<soap:binding style="document" transport="http://schemas.xmlsoap.org/soap/http"/>
		<!--===============================-->
		<!--===============================-->
		<wsdl:operation name="GetServiceCapabilities">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetServiceCapabilities"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetVideoSources">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdlGetVideoSources/"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<!--===============================-->
		<wsdl:operation name="GetAudioSources">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetAudioSources"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetAudioOutputs">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetAudioOutputs"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="CreateProfile">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/CreateProfile"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetProfile">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdlGetProfile/"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetProfiles">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetProfiles"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="AddVideoEncoderConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/AddVideoEncoderConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="AddVideoSourceConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/AddVideoSourceConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="AddAudioEncoderConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/AddAudioEncoderConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="AddAudioSourceConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/AddAudioSourceConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="AddPTZConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/AddPTZConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="AddVideoAnalyticsConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/AddVideoAnalyticsConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="AddMetadataConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/AddMetadataConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="AddAudioOutputConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/AddAudioOutputConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="AddAudioDecoderConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/AddAudioDecoderConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="RemoveVideoEncoderConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/RemoveVideoEncoderConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="RemoveVideoSourceConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/RemoveVideoSourceConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="RemoveAudioEncoderConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/RemoveAudioEncoderConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="RemoveAudioSourceConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/RemoveAudioSourceConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="RemovePTZConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/RemovePTZConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="RemoveVideoAnalyticsConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/RemoveVideoAnalyticsConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="RemoveMetadataConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/RemoveMetadataConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="RemoveAudioOutputConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/RemoveAudioOutputConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="RemoveAudioDecoderConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/RemoveAudioDecoderConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="DeleteProfile">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/DeleteProfile"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<!--===============================-->
		<!--===============================-->
		<wsdl:operation name="GetVideoSourceConfigurations">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetVideoSourceConfigurations"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetVideoEncoderConfigurations">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetVideoEncoderConfigurations"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetAudioSourceConfigurations">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdlGetAudioSourceConfigurations/"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetAudioEncoderConfigurations">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetAudioEncoderConfigurations"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetVideoAnalyticsConfigurations">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetVideoAnalyticsConfigurations"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetMetadataConfigurations">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetMetadataConfigurations"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetAudioOutputConfigurations">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetAudioOutputConfigurations"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetAudioDecoderConfigurations">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetAudioDecoderConfigurations"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetVideoSourceConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetVideoSourceConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetVideoEncoderConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetVideoEncoderConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetAudioSourceConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetAudioSourceConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetAudioEncoderConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetAudioEncoderConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetVideoAnalyticsConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetVideoAnalyticsConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetMetadataConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetMetadataConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetAudioOutputConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetAudioOutputConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetAudioDecoderConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetAudioDecoderConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetCompatibleVideoEncoderConfigurations">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetCompatibleVideoEncoderConfigurations"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetCompatibleVideoSourceConfigurations">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetCompatibleVideoSourceConfigurations"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetCompatibleAudioEncoderConfigurations">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetCompatibleAudioEncoderConfigurations"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetCompatibleAudioSourceConfigurations">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetCompatibleAudioSourceConfigurations"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetCompatibleVideoAnalyticsConfigurations">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetCompatibleVideoAnalyticsConfigurations"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetCompatibleMetadataConfigurations">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetCompatibleMetadataConfigurations"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetCompatibleAudioOutputConfigurations">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetCompatibleAudioOutputConfigurations"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetCompatibleAudioDecoderConfigurations">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetCompatibleAudioDecoderConfigurations"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="SetVideoSourceConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/SetVideoSourceConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetVideoEncoderConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/SetVideoEncoderConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetAudioSourceConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/SetAudioSourceConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetAudioEncoderConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/SetAudioEncoderConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetVideoAnalyticsConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/SetVideoAnalyticsConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetMetadataConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/SetMetadataConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetAudioOutputConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/SetAudioOutputConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetAudioDecoderConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/SetAudioDecoderConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetVideoSourceConfigurationOptions">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdlGetVideoSourceConfigurationOptions/"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetVideoEncoderConfigurationOptions">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetVideoEncoderConfigurationOptions"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetAudioSourceConfigurationOptions">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetAudioSourceConfigurationOptions"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetAudioEncoderConfigurationOptions">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetAudioEncoderConfigurationOptions"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetMetadataConfigurationOptions">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetMetadataConfigurationOptions"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetAudioOutputConfigurationOptions">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetAudioOutputConfigurationOptions"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetAudioDecoderConfigurationOptions">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetAudioDecoderConfigurationOptions"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetGuaranteedNumberOfVideoEncoderInstances">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetGuaranteedNumberOfVideoEncoderInstances"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<!--===============================-->
		<!--===============================-->
		<wsdl:operation name="GetStreamUri">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetStreamUri"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="StartMulticastStreaming">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/StartMulticastStreaming"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="StopMulticastStreaming">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/StopMulticastStreaming"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="SetSynchronizationPoint">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/SetSynchronizationPoint"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetSnapshotUri">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetSnapshotUri"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetVideoSourceModes">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetVideoSourceModes"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="SetVideoSourceMode">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/SetVideoSourceMode"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<!--==============OSD Operation Begin=================-->
		<wsdl:operation name="GetOSDs">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetOSDs"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetOSD">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetOSD"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetOSDOptions">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/GetOSDOptions"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetOSD">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/SetOSD"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="CreateOSD">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/CreateOSD"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="DeleteOSD">
			<soap:operation soapAction="http://www.onvif.org/ver10/media/wsdl/DeleteOSD"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--==============OSD Operation End=================-->
	</wsdl:binding>
</wsdl:definitions>
