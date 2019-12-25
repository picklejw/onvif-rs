<?xml version="1.0" encoding="utf-8"?>
<?xml-stylesheet type="text/xsl" href="../../../ver20/util/onvif-wsdl-viewer.xsl"?>
<!--
Copyright (c) 2008-2019 by ONVIF: Open Network Video Interface Forum. All rights reserved.

Recipients of this document may copy, distribute, publish, or display this document so long as this copyright notice, license and disclaimer are retained with all copies of the document. No license is granted to modify this document.

THIS DOCUMENT IS PROVIDED "AS IS," AND THE CORPORATION AND ITS MEMBERS AND THEIR AFFILIATES, MAKE NO REPRESENTATIONS OR WARRANTIES, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO, WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE, NON-INFRINGEMENT, OR TITLE; THAT THE CONTENTS OF THIS DOCUMENT ARE SUITABLE FOR ANY PURPOSE; OR THAT THE IMPLEMENTATION OF SUCH CONTENTS WILL NOT INFRINGE ANY PATENTS, COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS.
IN NO EVENT WILL THE CORPORATION OR ITS MEMBERS OR THEIR AFFILIATES BE LIABLE FOR ANY DIRECT, INDIRECT, SPECIAL, INCIDENTAL, PUNITIVE OR CONSEQUENTIAL DAMAGES, ARISING OUT OF OR RELATING TO ANY USE OR DISTRIBUTION OF THIS DOCUMENT, WHETHER OR NOT (1) THE CORPORATION, MEMBERS OR THEIR AFFILIATES HAVE BEEN ADVISED OF THE POSSIBILITY OF SUCH DAMAGES, OR (2) SUCH DAMAGES WERE REASONABLY FORESEEABLE, AND ARISING OUT OF OR RELATING TO ANY USE OR DISTRIBUTION OF THIS DOCUMENT.  THE FOREGOING DISCLAIMER AND LIMITATION ON LIABILITY DO NOT APPLY TO, INVALIDATE, OR LIMIT REPRESENTATIONS AND WARRANTIES MADE BY THE MEMBERS AND THEIR RESPECTIVE AFFILIATES TO THE CORPORATION AND OTHER MEMBERS IN CERTAIN WRITTEN POLICIES OF THE CORPORATION.
-->
<wsdl:definitions xmlns:wsdl="http://schemas.xmlsoap.org/wsdl/" xmlns:soap="http://schemas.xmlsoap.org/wsdl/soap12/" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:tr2="http://www.onvif.org/ver20/media/wsdl" targetNamespace="http://www.onvif.org/ver20/media/wsdl">
	<wsdl:types>
		<xs:schema targetNamespace="http://www.onvif.org/ver20/media/wsdl" xmlns:tt="http://www.onvif.org/ver10/schema" xmlns:xs="http://www.w3.org/2001/XMLSchema" elementFormDefault="qualified" version="19.06">
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
						<xs:element name="Capabilities" type="tr2:Capabilities2">
							<xs:annotation>
								<xs:documentation>The capabilities for the media service is returned in the Capabilities element.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:complexType name="Capabilities2">
				<xs:sequence>
					<xs:element name="ProfileCapabilities" type="tr2:ProfileCapabilities">
						<xs:annotation>
							<xs:documentation>Media profile capabilities.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="StreamingCapabilities" type="tr2:StreamingCapabilities">
						<xs:annotation>
							<xs:documentation>Streaming capabilities.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>   <!-- first ONVIF then Vendor -->
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
				<xs:attribute name="Mask" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates if Masking is supported.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="SourceMask" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates that privacy masks are only supported at the video source level and not the video source configuration level. 
							If this is true any addition, deletion or change of a privacy mask done for one video source configuration will automatically be 
							applied by the device to a corresponding privacy mask for all other video source configuration associated with the same video source.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<xs:element name="Capabilities" type="tr2:Capabilities2"/>
			<!--===============================-->
			<xs:complexType name="ProfileCapabilities">
				<xs:sequence>
					<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>   <!-- first ONVIF then Vendor -->
				</xs:sequence>
				<xs:attribute name="MaximumNumberOfProfiles" type="xs:int">
					<xs:annotation>
						<xs:documentation>Maximum number of profiles supported.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="ConfigurationsSupported" type="tt:StringAttrList">
					<xs:annotation>
						<xs:documentation>The configurations supported by the device as defined by tr2:ConfigurationEnumeration. The enumeration value "All" shall not be included in this list.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<!--===============================-->
			<xs:complexType name="StreamingCapabilities">
				<xs:sequence>
					<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>   <!-- first ONVIF then Vendor -->
				</xs:sequence>
				<xs:attribute name="RTSPStreaming" type="xs:boolean">
					<xs:annotation>
						<xs:documentation> Indicates support for live media streaming via RTSP.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="RTPMulticast" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for RTP multicast.</xs:documentation>
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
				<xs:attribute name="RTSPWebSocketUri" type="xs:anyURI">
					<xs:annotation>
						<xs:documentation>If streaming over WebSocket is supported, this shall return the RTSP WebSocket URI as described in Streaming Specification Section 5.1.1.5.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="AutoStartMulticast" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for non-RTSP controlled multicast streaming.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<!--===============================-->
			<xs:simpleType name="ConfigurationEnumeration">
				<xs:restriction base="xs:string">
					<xs:enumeration value="All"/> 			<!-- For matching all configurations -->
					<xs:enumeration value="VideoSource"/>
					<xs:enumeration value="VideoEncoder"/>
					<xs:enumeration value="AudioSource"/>
					<xs:enumeration value="AudioEncoder"/>
					<xs:enumeration value="AudioOutput"/>
					<xs:enumeration value="AudioDecoder"/>
					<xs:enumeration value="Metadata"/>
					<xs:enumeration value="Analytics"/>
					<xs:enumeration value="PTZ"/>
					<!-- Vendors may conceptually add items here prefixing them with there own namespace -->
				</xs:restriction>
			</xs:simpleType>
			<xs:element name="ConfigurationEnumeration" type="tr2:ConfigurationEnumeration"/>
			<xs:complexType name="ConfigurationRef">
				<xs:sequence>
					<xs:element name="Type" type="xs:string">
						<xs:annotation>
							<xs:documentation>Type of the configuration as defined by tr2:ConfigurationEnumeration.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="Token" type="tt:ReferenceToken" minOccurs="0">
						<xs:annotation>
							<xs:documentation>Reference token of an existing configuration.
							Token shall be included in the AddConfiguration request along with the type.
							Token shall be included in the CreateProfile request when Configuration elements are included and type is selected.
							Token is optional for RemoveConfiguration request. If no token is provided in RemoveConfiguration request, device shall
							remove the configuration of the type included in the profile.</xs:documentation>
						</xs:annotation>
					</xs:element>
				</xs:sequence>
			</xs:complexType>
			<!--===============================-->
			<xs:complexType name="ConfigurationSet">
				<xs:annotation>
					<xs:documentation>
						A set of media configurations. 
					</xs:documentation>
				</xs:annotation>
				<xs:sequence>
					<xs:element name="VideoSource" type="tt:VideoSourceConfiguration" minOccurs="0">
						<xs:annotation>
							<xs:documentation>Optional configuration of the Video input.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="AudioSource" type="tt:AudioSourceConfiguration" minOccurs="0">
						<xs:annotation>
							<xs:documentation>Optional configuration of the Audio input.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="VideoEncoder" type="tt:VideoEncoder2Configuration" minOccurs="0">
						<xs:annotation>
							<xs:documentation>Optional configuration of the Video encoder.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="AudioEncoder" type="tt:AudioEncoder2Configuration" minOccurs="0">
						<xs:annotation>
							<xs:documentation>Optional configuration of the Audio encoder.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="Analytics" type="tt:VideoAnalyticsConfiguration" minOccurs="0">
						<xs:annotation>
							<xs:documentation>Optional configuration of the analytics module and rule engine.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="PTZ" type="tt:PTZConfiguration" minOccurs="0">
						<xs:annotation>
							<xs:documentation>Optional configuration of the pan tilt zoom unit.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="Metadata" type="tt:MetadataConfiguration" minOccurs="0">
						<xs:annotation>
							<xs:documentation>Optional configuration of the metadata stream.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="AudioOutput" type="tt:AudioOutputConfiguration" minOccurs="0">
						<xs:annotation>
							<xs:documentation>Optional configuration of the Audio output.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="AudioDecoder" type="tt:AudioDecoderConfiguration" minOccurs="0">
						<xs:annotation>
							<xs:documentation>Optional configuration of the Audio decoder.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>   <!-- first ONVIF then Vendor -->
				</xs:sequence>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<!--===============================-->
			<xs:complexType name="MediaProfile">
				<xs:annotation>
					<xs:documentation>
						A media profile consists of a set of media configurations. 
					</xs:documentation>
				</xs:annotation>
				<xs:sequence>
					<xs:element name="Name" type="tt:Name">
						<xs:annotation>
							<xs:documentation>User readable name of the profile.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="Configurations" type="tr2:ConfigurationSet" minOccurs="0">
						<xs:annotation>
							<xs:documentation>The configurations assigned to the profile.</xs:documentation>
						</xs:annotation>
					</xs:element>
				</xs:sequence>
				<xs:attribute name="token" type="tt:ReferenceToken" use="required">
					<xs:annotation>
						<xs:documentation>Unique identifier of the profile.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="fixed" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>A value of true signals that the profile cannot be deleted. Default is false.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<!--===============================-->
			<xs:element name="CreateProfile">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Name" type="tt:Name">
							<xs:annotation>
								<xs:documentation>friendly name of the profile to be created</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="Configuration" type="tr2:ConfigurationRef" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>Optional set of configurations to be assigned to the profile. List entries with tr2:ConfigurationEnumeration value "All" shall be ignored.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="CreateProfileResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Token" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Token assigned by the device for the newly created profile.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetProfiles">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Token" type="tt:ReferenceToken" minOccurs="0">
							<xs:annotation>
								<xs:documentation>Optional token of the requested profile.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="Type" type="xs:string" maxOccurs="unbounded" minOccurs="0">
							<xs:annotation>
								<xs:documentation>The types shall be provided as defined by tr2:ConfigurationEnumeration.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetProfilesResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Profiles" type="tr2:MediaProfile" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>Lists all profiles that exist in the media service. The response provides the requested types of Configurations as far as available. 
									If a profile doesn't contain a type no error shall be provided.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="AddConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Reference to the profile where the configuration should be added</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="Name" type="tt:Name" minOccurs="0">
							<xs:annotation>
								<xs:documentation>Optional item. If present updates the Name property of the profile.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="Configuration" type="tr2:ConfigurationRef" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>
									List of configurations to be added. The types shall be provided in the order defined by tr2:ConfigurationEnumeration. List entries with tr2:ConfigurationEnumeration value "All" shall be ignored.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="AddConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
				   </xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="RemoveConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ProfileToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>This element contains a  reference to the media profile from which the AudioDecoderConfiguration shall be removed.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="Configuration" type="tr2:ConfigurationRef" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>List of configurations to be removed. The types shall be provided in the order defined by tr2:ConfigurationEnumeration. Tokens appearing in the configuration list shall be ignored. Presence of the "All" type shall result in an empty profile.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="RemoveConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
    				</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="DeleteProfile">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Token" type="tt:ReferenceToken">
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
			<!--	Generic request body for all GetConfigurationXxx and GetConfigurationXxxOptions commands	-->
			<xs:complexType name="GetConfiguration">
				<xs:sequence>
					<xs:element name="ConfigurationToken" type="tt:ReferenceToken" minOccurs="0">
						<xs:annotation>
							<xs:documentation>Token of the requested configuration.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="ProfileToken" type="tt:ReferenceToken" minOccurs="0">
						<xs:annotation>
							<xs:documentation>Contains the token of an existing media profile the configurations shall be compatible with.</xs:documentation>
						</xs:annotation>
					</xs:element>
				</xs:sequence>
			</xs:complexType>
			
			<xs:element name="GetVideoEncoderConfigurations" type="tr2:GetConfiguration"/>
			<xs:element name="GetVideoEncoderConfigurationsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configurations" type="tt:VideoEncoder2Configuration" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>This element contains a list of video encoder configurations.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetVideoSourceConfigurations" type="tr2:GetConfiguration"/>
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
			<xs:element name="GetAudioEncoderConfigurations" type="tr2:GetConfiguration"/>
			<xs:element name="GetAudioEncoderConfigurationsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configurations" type="tt:AudioEncoder2Configuration" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>This element contains a list of audio encoder configurations.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetAudioSourceConfigurations" type="tr2:GetConfiguration"/>
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
			<xs:element name="GetAnalyticsConfigurations" type="tr2:GetConfiguration"/>
			<xs:element name="GetAnalyticsConfigurationsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configurations" type="tt:VideoAnalyticsConfiguration" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>This element contains a list of Analytics configurations.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetMetadataConfigurations" type="tr2:GetConfiguration"/>
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
			<xs:element name="GetAudioOutputConfigurations" type="tr2:GetConfiguration"/>
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
			<xs:element name="GetAudioDecoderConfigurations" type="tr2:GetConfiguration"/>
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
			<xs:element name="SetVideoEncoderConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configuration" type="tt:VideoEncoder2Configuration">
							<xs:annotation>
								<xs:documentation>Contains the modified video encoder configuration. The configuration shall exist in the device.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:complexType name="SetConfigurationResponse">
				<xs:sequence>
				</xs:sequence>
			</xs:complexType>
			<xs:element name="SetVideoEncoderConfigurationResponse" type="tr2:SetConfigurationResponse"/>
						<xs:element name="SetVideoSourceConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configuration" type="tt:VideoSourceConfiguration">
							<xs:annotation>
								<xs:documentation>Contains the modified video source configuration. The configuration shall exist in the device.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetVideoSourceConfigurationResponse" type="tr2:SetConfigurationResponse"/>
						<xs:element name="SetAudioEncoderConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configuration" type="tt:AudioEncoder2Configuration">
							<xs:annotation>
								<xs:documentation>Contains the modified audio encoder configuration. The configuration shall exist in the device.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetAudioEncoderConfigurationResponse" type="tr2:SetConfigurationResponse"/>
			<xs:element name="SetAudioSourceConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configuration" type="tt:AudioSourceConfiguration">
							<xs:annotation>
								<xs:documentation>Contains the modified audio source configuration. The configuration shall exist in the device.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetAudioSourceConfigurationResponse" type="tr2:SetConfigurationResponse"/>
			<xs:element name="SetMetadataConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configuration" type="tt:MetadataConfiguration">
							<xs:annotation>
								<xs:documentation>Contains the modified metadata configuration. The configuration shall exist in the device.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetMetadataConfigurationResponse" type="tr2:SetConfigurationResponse"/>
			<xs:element name="SetAudioOutputConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configuration" type="tt:AudioOutputConfiguration">
							<xs:annotation>
								<xs:documentation>Contains the modified audio output configuration. The configuration shall exist in the device.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetAudioOutputConfigurationResponse" type="tr2:SetConfigurationResponse"/>
			<xs:element name="SetAudioDecoderConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configuration" type="tt:AudioDecoderConfiguration">
							<xs:annotation>
								<xs:documentation>Contains the modified audio decoder configuration. The configuration shall exist in the device.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetAudioDecoderConfigurationResponse" type="tr2:SetConfigurationResponse"/>
			<!--===============================-->
			<xs:element name="GetVideoSourceConfigurationOptions" type="tr2:GetConfiguration"/>
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
			<xs:element name="GetVideoEncoderConfigurationOptions" type="tr2:GetConfiguration"/>
			<xs:element name="GetVideoEncoderConfigurationOptionsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Options" type="tt:VideoEncoder2ConfigurationOptions" maxOccurs="unbounded"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetAudioSourceConfigurationOptions" type="tr2:GetConfiguration"/>
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
			<xs:element name="GetAudioEncoderConfigurationOptions" type="tr2:GetConfiguration"/>
			<xs:element name="GetAudioEncoderConfigurationOptionsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Options" type="tt:AudioEncoder2ConfigurationOptions" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>This message contains the audio encoder configuration options. If a audio encoder configuration is specified, the options shall concern that particular configuration. If a media profile is specified, the options shall be compatible with that media profile. If no tokens are specified, the options shall be considered generic for the device.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetMetadataConfigurationOptions" type="tr2:GetConfiguration"/>
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
			<xs:element name="GetAudioOutputConfigurationOptions" type="tr2:GetConfiguration"/>
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
			<xs:element name="GetAudioDecoderConfigurationOptions" type="tr2:GetConfiguration"/>
			<xs:element name="GetAudioDecoderConfigurationOptionsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Options" type="tt:AudioEncoder2ConfigurationOptions" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>This message contains the audio decoder configuration options. If a audio decoder configuration is specified, the options shall concern that particular configuration. If a media profile is specified, the options shall be compatible with that media profile. If no tokens are specified, the options shall be considered generic for the device.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<!--===============================-->
			<xs:simpleType name="TransportProtocol">
				<xs:restriction base="xs:string">
					<xs:enumeration value="RtspUnicast"/>		<!-- RTSP straming RTP as UDP Unicast. -->
					<xs:enumeration value="RtspMulticast"/>		<!-- RTSP straming RTP as UDP Multicast. -->
					<xs:enumeration value="RTSP"/>				<!-- RTSP straming RTP over TCP. -->
					<xs:enumeration value="RtspOverHttp"/>		<!-- Tunneling both the RTSP control channel and the RTP stream over HTTP or HTTPS. -->
				</xs:restriction>
			</xs:simpleType>
			<!--===============================-->
			<xs:element name="GetVideoEncoderInstances">
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
			<!--===============================-->
			<xs:complexType name="EncoderInstance">
				<xs:sequence>
					<xs:element name="Encoding" type="xs:string">
						<xs:annotation>
					<xs:documentation>Video Media Subtype for the video format. For definitions see tt:VideoEncodingMimeNames and <a href="https://www.iana.org/assignments/media-types/media-types.xhtml#video"> IANA Media Types</a>.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="Number" type="xs:int">
						<xs:annotation>
							<xs:documentation>The minimum guaranteed number of encoder instances (applications) for the VideoSourceConfiguration.</xs:documentation>
						</xs:annotation>
					</xs:element>
				</xs:sequence>
						<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<xs:complexType name="EncoderInstanceInfo">
				<xs:sequence>
					<xs:element name="Codec" type="tr2:EncoderInstance" minOccurs="0" maxOccurs="unbounded">
						<xs:annotation>
							<xs:documentation>If a device limits the number of instances for respective Video Codecs the response contains the information how many streams can be set up at the same time per VideoSource.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="Total" type="xs:int">
						<xs:annotation>
							<xs:documentation>The minimum guaranteed total number of encoder instances (applications) per VideoSourceConfiguration. The device is able to deliver the Total number of streams</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>   <!-- first ONVIF then Vendor -->
				</xs:sequence>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<xs:element name="GetVideoEncoderInstancesResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Info" type="tr2:EncoderInstanceInfo">
							<xs:annotation>
								<xs:documentation>The minimum guaranteed total number of encoder instances (applications) per VideoSourceConfiguration.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetStreamUri">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Protocol" type="xs:string">
							<xs:annotation>
								<xs:documentation>Defines the network protocol for streaming as defined by tr2:TransportProtocol</xs:documentation>
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
						<xs:element name="Uri" type="xs:anyURI">
							<xs:annotation>
								<xs:documentation>Stable Uri to be used for requesting the media stream</xs:documentation>
							</xs:annotation>
						</xs:element>
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
						<xs:element name="Uri" type="xs:anyURI">
							<xs:annotation>
								<xs:documentation>Stable Uri to be used for requesting snapshot images.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:complexType name="StartStopMulticastStreaming">
				<xs:sequence>
					<xs:element name="ProfileToken" type="tt:ReferenceToken">
						<xs:annotation>
							<xs:documentation>Contains the token of the Profile that is used to define the multicast stream.</xs:documentation>
						</xs:annotation>
					</xs:element>
				</xs:sequence>
			</xs:complexType>
			<xs:element name="StartMulticastStreaming" type="tr2:StartStopMulticastStreaming"/>
			<xs:element name="StartMulticastStreamingResponse" type="tr2:SetConfigurationResponse"/>
			<xs:element name="StopMulticastStreaming" type="tr2:StartStopMulticastStreaming"/>
			<xs:element name="StopMulticastStreamingResponse" type="tr2:SetConfigurationResponse"/>
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
						<xs:element name="VideoSourceModes" type="tr2:VideoSourceMode" maxOccurs="unbounded">
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
					<xs:documentation>List of one or more encodings supported for this video source.  For name definitions see tt:VideoEncodingMimeNames, and see <a href="https://www.iana.org/assignments/media-types/media-types.xhtml#video">IANA Media Types</a>.</xs:documentation>
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
					<xs:element name="Encodings" type="tr2:EncodingTypes">
						<xs:annotation>
							<xs:documentation>List of one or more encodings supported for this video source.  For name definitions see tt:VideoEncodingMimeNames, and see <a href="https://www.iana.org/assignments/media-types/media-types.xhtml#video">IANA Media Types</a>.</xs:documentation>
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
					<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>   <!-- first ONVIF then Vendor -->
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
			<!--============OSD Schema Begin================-->
			<xs:element name="GetOSDs">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="OSDToken" type="tt:ReferenceToken" minOccurs="0">
							<xs:annotation>
								<xs:documentation>The GetOSDs command fetches the OSD configuration if the OSD token is known.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="ConfigurationToken" type="tt:ReferenceToken" minOccurs="0">
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
			<xs:element name="SetOSD">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="OSD" type="tt:OSDConfiguration">
							<xs:annotation>
								<xs:documentation>Contains the modified OSD configuration.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetOSDResponse" type="tr2:SetConfigurationResponse"/>
			<!--===============================-->
			<xs:element name="GetOSDOptions">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ConfigurationToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Video Source Configuration Token that specifies an existing video source configuration that the options shall be compatible with.</xs:documentation>
							</xs:annotation>
						</xs:element>
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
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="DeleteOSDResponse" type="tr2:SetConfigurationResponse"/>
			<!--============OSD Schema End================-->
			<!--============Masking Schema Begin================-->
			<xs:simpleType name="MaskType">
				<xs:restriction base="xs:string">
					<xs:enumeration value="Color"/>
					<xs:enumeration value="Pixelated"/>
					<xs:enumeration value="Blurred"/>
				</xs:restriction>
			</xs:simpleType>
			<xs:complexType name="Mask">
				<xs:sequence>
					<xs:element name="ConfigurationToken" type="tt:ReferenceToken">
						<xs:annotation>
							<xs:documentation>Token of the VideoSourceConfiguration the Mask is associated with.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="Polygon" type="tt:Polygon">
						<xs:annotation><xs:documentation>Geometric representation of the mask area.</xs:documentation></xs:annotation>
					</xs:element>
					<xs:element name="Type" type="xs:string">
						<xs:annotation><xs:documentation>Type of masking as defined by tr2:MaskType:
							<ul>
								<li>Color - The masked area is colored with color defined by the Color field.</li>
								<li>Pixelated - The masked area is filled in mosaic style to hide details.</li>
								<li>Blurred - The masked area is low pass filtered to hide details.</li>
							</ul></xs:documentation></xs:annotation>
					</xs:element>
					<xs:element name="Color" type="tt:Color" minOccurs="0">
						<xs:annotation><xs:documentation>Color of the masked area.</xs:documentation></xs:annotation>
					</xs:element>
					<xs:element name="Enabled" type="xs:boolean">
						<xs:annotation><xs:documentation>If set the mask will cover the image, otherwise it will be fully transparent.</xs:documentation></xs:annotation>
					</xs:element>
					<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>   <!-- first ONVIF then Vendor -->
				</xs:sequence>
				<xs:attribute name="token" type="tt:ReferenceToken">
					<xs:annotation>
						<xs:documentation>Token of the mask.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:anyAttribute/>
			</xs:complexType>
			<xs:element name="GetMasks">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Token" type="tt:ReferenceToken" minOccurs="0">
							<xs:annotation>
								<xs:documentation>Optional mask token of an existing mask.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="ConfigurationToken" type="tt:ReferenceToken" minOccurs="0">
							<xs:annotation>
								<xs:documentation>Optional token of a Video Source Configuration.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetMasksResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Masks" type="tr2:Mask" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>List of Mask configurations.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetMask">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Mask" type="tr2:Mask">
							<xs:annotation>
								<xs:documentation>Mask to be updated.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetMaskResponse" type="tr2:SetConfigurationResponse"/>
			<!--===============================-->
			<xs:element name="GetMaskOptions">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ConfigurationToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Video Source Configuration Token that specifies an existing video source configuration that the options shall be compatible with.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			
			<xs:complexType name="MaskOptions">
				<xs:sequence>
					<xs:element name="MaxMasks" type="xs:int">
						<xs:annotation>
							<xs:documentation>Maximum supported number of masks per VideoSourceConfiguration.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="MaxPoints" type="xs:int">
						<xs:annotation>
							<xs:documentation>Maximum supported number of points per mask.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="Types" type="xs:string" maxOccurs="unbounded">
						<xs:annotation>
							<xs:documentation>Information which types of tr2:MaskType are supported. Valid values are 'Color', 'Pixelated' and 'Blurred'.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="Color" type="tt:ColorOptions">
						<xs:annotation>
							<xs:documentation>Colors supported.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>   <!-- first ONVIF then Vendor -->
				</xs:sequence>
				<xs:attribute name="RectangleOnly" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Information whether the polygon must have four points and a rectangular shape.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="SingleColorOnly" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates the device capability of change in color of privacy mask for one video source configuration will automatically be applied to all the privacy masks associated with the same video source configuration.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			
			<xs:element name="GetMaskOptionsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Options" type="tr2:MaskOptions">
							<xs:annotation>
								<xs:documentation/>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="CreateMask">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Mask" type="tr2:Mask">
							<xs:annotation>
								<xs:documentation>Contain the initial mask configuration for create.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="CreateMaskResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Token" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Returns Token of the newly created Mask</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="DeleteMask">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Token" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>This element contains a reference to the Mask configuration that should be deleted.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="DeleteMaskResponse" type="tr2:SetConfigurationResponse"/>
			<!--============Mask Schema End================-->
			<!--============ Schema End================-->
		</xs:schema>
	</wsdl:types>
	<wsdl:message name="GetServiceCapabilitiesRequest">
		<wsdl:part name="parameters" element="tr2:GetServiceCapabilities"/>
	</wsdl:message>
	<wsdl:message name="GetServiceCapabilitiesResponse">
		<wsdl:part name="parameters" element="tr2:GetServiceCapabilitiesResponse"/>
	</wsdl:message>
	<wsdl:message name="CreateProfileRequest">
		<wsdl:part name="parameters" element="tr2:CreateProfile"/>
	</wsdl:message>
	<wsdl:message name="CreateProfileResponse">
		<wsdl:part name="parameters" element="tr2:CreateProfileResponse"/>
	</wsdl:message>
	<wsdl:message name="GetProfilesRequest">
		<wsdl:part name="parameters" element="tr2:GetProfiles"/>
	</wsdl:message>
	<wsdl:message name="GetProfilesResponse">
		<wsdl:part name="parameters" element="tr2:GetProfilesResponse"/>
	</wsdl:message>
	<wsdl:message name="AddConfigurationRequest">
		<wsdl:part name="parameters" element="tr2:AddConfiguration"/>
	</wsdl:message>
	<wsdl:message name="AddConfigurationResponse">
		<wsdl:part name="parameters" element="tr2:AddConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="RemoveConfigurationRequest">
		<wsdl:part name="parameters" element="tr2:RemoveConfiguration"/>
	</wsdl:message>
	<wsdl:message name="RemoveConfigurationResponse">
		<wsdl:part name="parameters" element="tr2:RemoveConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="DeleteProfileRequest">
		<wsdl:part name="parameters" element="tr2:DeleteProfile"/>
	</wsdl:message>
	<wsdl:message name="DeleteProfileResponse">
		<wsdl:part name="parameters" element="tr2:DeleteProfileResponse"/>
	</wsdl:message>
	<!--===============================-->
	<wsdl:message name="GetVideoSourceConfigurationsRequest">
		<wsdl:part name="parameters" element="tr2:GetVideoSourceConfigurations"/>
	</wsdl:message>
	<wsdl:message name="GetVideoSourceConfigurationsResponse">
		<wsdl:part name="parameters" element="tr2:GetVideoSourceConfigurationsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetVideoEncoderConfigurationsRequest">
		<wsdl:part name="parameters" element="tr2:GetVideoEncoderConfigurations"/>
	</wsdl:message>
	<wsdl:message name="GetVideoEncoderConfigurationsResponse">
		<wsdl:part name="parameters" element="tr2:GetVideoEncoderConfigurationsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetAudioSourceConfigurationsRequest">
		<wsdl:part name="parameters" element="tr2:GetAudioSourceConfigurations"/>
	</wsdl:message>
	<wsdl:message name="GetAudioSourceConfigurationsResponse">
		<wsdl:part name="parameters" element="tr2:GetAudioSourceConfigurationsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetAudioEncoderConfigurationsRequest">
		<wsdl:part name="parameters" element="tr2:GetAudioEncoderConfigurations"/>
	</wsdl:message>
	<wsdl:message name="GetAudioEncoderConfigurationsResponse">
		<wsdl:part name="parameters" element="tr2:GetAudioEncoderConfigurationsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetAnalyticsConfigurationsRequest">
		<wsdl:part name="parameters" element="tr2:GetAnalyticsConfigurations"/>
	</wsdl:message>
	<wsdl:message name="GetAnalyticsConfigurationsResponse">
		<wsdl:part name="parameters" element="tr2:GetAnalyticsConfigurationsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetMetadataConfigurationsRequest">
		<wsdl:part name="parameters" element="tr2:GetMetadataConfigurations"/>
	</wsdl:message>
	<wsdl:message name="GetMetadataConfigurationsResponse">
		<wsdl:part name="parameters" element="tr2:GetMetadataConfigurationsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetAudioOutputConfigurationsRequest">
		<wsdl:part name="parameters" element="tr2:GetAudioOutputConfigurations"/>
	</wsdl:message>
	<wsdl:message name="GetAudioOutputConfigurationsResponse">
		<wsdl:part name="parameters" element="tr2:GetAudioOutputConfigurationsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetAudioDecoderConfigurationsRequest">
		<wsdl:part name="parameters" element="tr2:GetAudioDecoderConfigurations"/>
	</wsdl:message>
	<wsdl:message name="GetAudioDecoderConfigurationsResponse">
		<wsdl:part name="parameters" element="tr2:GetAudioDecoderConfigurationsResponse"/>
	</wsdl:message>
	<!--===============================-->
	<wsdl:message name="SetVideoSourceConfigurationRequest">
		<wsdl:part name="parameters" element="tr2:SetVideoSourceConfiguration"/>
	</wsdl:message>
	<wsdl:message name="SetVideoSourceConfigurationResponse">
		<wsdl:part name="parameters" element="tr2:SetVideoSourceConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="SetVideoEncoderConfigurationRequest">
		<wsdl:part name="parameters" element="tr2:SetVideoEncoderConfiguration"/>
	</wsdl:message>
	<wsdl:message name="SetVideoEncoderConfigurationResponse">
		<wsdl:part name="parameters" element="tr2:SetVideoEncoderConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="SetAudioSourceConfigurationRequest">
		<wsdl:part name="parameters" element="tr2:SetAudioSourceConfiguration"/>
	</wsdl:message>
	<wsdl:message name="SetAudioSourceConfigurationResponse">
		<wsdl:part name="parameters" element="tr2:SetAudioSourceConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="SetAudioEncoderConfigurationRequest">
		<wsdl:part name="parameters" element="tr2:SetAudioEncoderConfiguration"/>
	</wsdl:message>
	<wsdl:message name="SetAudioEncoderConfigurationResponse">
		<wsdl:part name="parameters" element="tr2:SetAudioEncoderConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="SetMetadataConfigurationRequest">
		<wsdl:part name="parameters" element="tr2:SetMetadataConfiguration"/>
	</wsdl:message>
	<wsdl:message name="SetMetadataConfigurationResponse">
		<wsdl:part name="parameters" element="tr2:SetMetadataConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="SetAudioOutputConfigurationRequest">
		<wsdl:part name="parameters" element="tr2:SetAudioOutputConfiguration"/>
	</wsdl:message>
	<wsdl:message name="SetAudioOutputConfigurationResponse">
		<wsdl:part name="parameters" element="tr2:SetAudioOutputConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="SetAudioDecoderConfigurationRequest">
		<wsdl:part name="parameters" element="tr2:SetAudioDecoderConfiguration"/>
	</wsdl:message>
	<wsdl:message name="SetAudioDecoderConfigurationResponse">
		<wsdl:part name="parameters" element="tr2:SetAudioDecoderConfigurationResponse"/>
	</wsdl:message>
	<!--===============================-->
	<wsdl:message name="GetVideoSourceConfigurationOptionsRequest">
		<wsdl:part name="parameters" element="tr2:GetVideoSourceConfigurationOptions"/>
	</wsdl:message>
	<wsdl:message name="GetVideoSourceConfigurationOptionsResponse">
		<wsdl:part name="parameters" element="tr2:GetVideoSourceConfigurationOptionsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetVideoEncoderConfigurationOptionsRequest">
		<wsdl:part name="parameters" element="tr2:GetVideoEncoderConfigurationOptions"/>
	</wsdl:message>
	<wsdl:message name="GetVideoEncoderConfigurationOptionsResponse">
		<wsdl:part name="parameters" element="tr2:GetVideoEncoderConfigurationOptionsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetAudioSourceConfigurationOptionsRequest">
		<wsdl:part name="parameters" element="tr2:GetAudioSourceConfigurationOptions"/>
	</wsdl:message>
	<wsdl:message name="GetAudioSourceConfigurationOptionsResponse">
		<wsdl:part name="parameters" element="tr2:GetAudioSourceConfigurationOptionsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetAudioEncoderConfigurationOptionsRequest">
		<wsdl:part name="parameters" element="tr2:GetAudioEncoderConfigurationOptions"/>
	</wsdl:message>
	<wsdl:message name="GetAudioEncoderConfigurationOptionsResponse">
		<wsdl:part name="parameters" element="tr2:GetAudioEncoderConfigurationOptionsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetMetadataConfigurationOptionsRequest">
		<wsdl:part name="parameters" element="tr2:GetMetadataConfigurationOptions"/>
	</wsdl:message>
	<wsdl:message name="GetMetadataConfigurationOptionsResponse">
		<wsdl:part name="parameters" element="tr2:GetMetadataConfigurationOptionsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetAudioOutputConfigurationOptionsRequest">
		<wsdl:part name="parameters" element="tr2:GetAudioOutputConfigurationOptions"/>
	</wsdl:message>
	<wsdl:message name="GetAudioOutputConfigurationOptionsResponse">
		<wsdl:part name="parameters" element="tr2:GetAudioOutputConfigurationOptionsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetAudioDecoderConfigurationOptionsRequest">
		<wsdl:part name="parameters" element="tr2:GetAudioDecoderConfigurationOptions"/>
	</wsdl:message>
	<wsdl:message name="GetAudioDecoderConfigurationOptionsResponse">
		<wsdl:part name="parameters" element="tr2:GetAudioDecoderConfigurationOptionsResponse"/>
	</wsdl:message>
	<!--===============================-->
	<wsdl:message name="GetVideoEncoderInstancesRequest">
		<wsdl:part name="parameters" element="tr2:GetVideoEncoderInstances"/>
	</wsdl:message>
	<wsdl:message name="GetVideoEncoderInstancesResponse">
		<wsdl:part name="parameters" element="tr2:GetVideoEncoderInstancesResponse"/>
	</wsdl:message>
	<wsdl:message name="GetStreamUriRequest">
		<wsdl:part name="parameters" element="tr2:GetStreamUri"/>
	</wsdl:message>
	<wsdl:message name="GetStreamUriResponse">
		<wsdl:part name="parameters" element="tr2:GetStreamUriResponse"/>
	</wsdl:message>
	<wsdl:message name="SetSynchronizationPointRequest">
		<wsdl:part name="parameters" element="tr2:SetSynchronizationPoint"/>
	</wsdl:message>
	<wsdl:message name="SetSynchronizationPointResponse">
		<wsdl:part name="parameters" element="tr2:SetSynchronizationPointResponse"/>
	</wsdl:message>
	<wsdl:message name="GetSnapshotUriRequest">
		<wsdl:part name="parameters" element="tr2:GetSnapshotUri"/>
	</wsdl:message>
	<wsdl:message name="GetSnapshotUriResponse">
		<wsdl:part name="parameters" element="tr2:GetSnapshotUriResponse"/>
	</wsdl:message>
	<wsdl:message name="StartMulticastStreamingRequest">
		<wsdl:part name="parameters" element="tr2:StartMulticastStreaming"/>
	</wsdl:message>
	<wsdl:message name="StopMulticastStreamingRequest">
		<wsdl:part name="parameters" element="tr2:StopMulticastStreaming"/>
	</wsdl:message>
	<wsdl:message name="StopMulticastStreamingResponse">
		<wsdl:part name="parameters" element="tr2:StopMulticastStreamingResponse"/>
	</wsdl:message>
	<wsdl:message name="StartMulticastStreamingResponse">
		<wsdl:part name="parameters" element="tr2:StartMulticastStreamingResponse"/>
	</wsdl:message>
	<wsdl:message name="GetVideoSourceModesRequest">
		<wsdl:part name="parameters" element="tr2:GetVideoSourceModes"/>
	</wsdl:message>
	<wsdl:message name="GetVideoSourceModesResponse">
		<wsdl:part name="parameters" element="tr2:GetVideoSourceModesResponse"/>
	</wsdl:message>
	<wsdl:message name="SetVideoSourceModeRequest">
		<wsdl:part name="parameters" element="tr2:SetVideoSourceMode"/>
	</wsdl:message>
	<wsdl:message name="SetVideoSourceModeResponse">
		<wsdl:part name="parameters" element="tr2:SetVideoSourceModeResponse"/>
	</wsdl:message>
	<wsdl:message name="GetOSDsRequest">
		<wsdl:part name="parameters" element="tr2:GetOSDs"/>
	</wsdl:message>
	<wsdl:message name="GetOSDsResponse">
		<wsdl:part name="parameters" element="tr2:GetOSDsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetOSDOptionsRequest">
		<wsdl:part name="parameters" element="tr2:GetOSDOptions"/>
	</wsdl:message>
	<wsdl:message name="GetOSDOptionsResponse">
		<wsdl:part name="parameters" element="tr2:GetOSDOptionsResponse"/>
	</wsdl:message>
	<wsdl:message name="SetOSDRequest">
		<wsdl:part name="parameters" element="tr2:SetOSD"/>
	</wsdl:message>
	<wsdl:message name="SetOSDResponse">
		<wsdl:part name="parameters" element="tr2:SetOSDResponse"/>
	</wsdl:message>
	<wsdl:message name="CreateOSDRequest">
		<wsdl:part name="parameters" element="tr2:CreateOSD"/>
	</wsdl:message>
	<wsdl:message name="CreateOSDResponse">
		<wsdl:part name="parameters" element="tr2:CreateOSDResponse"/>
	</wsdl:message>
	<wsdl:message name="DeleteOSDRequest">
		<wsdl:part name="parameters" element="tr2:DeleteOSD"/>
	</wsdl:message>
	<wsdl:message name="DeleteOSDResponse">
		<wsdl:part name="parameters" element="tr2:DeleteOSDResponse"/>
	</wsdl:message>
	<wsdl:message name="GetMasksRequest">
		<wsdl:part name="parameters" element="tr2:GetMasks"/>
	</wsdl:message>
	<wsdl:message name="GetMasksResponse">
		<wsdl:part name="parameters" element="tr2:GetMasksResponse"/>
	</wsdl:message>
	<wsdl:message name="GetMaskOptionsRequest">
		<wsdl:part name="parameters" element="tr2:GetMaskOptions"/>
	</wsdl:message>
	<wsdl:message name="GetMaskOptionsResponse">
		<wsdl:part name="parameters" element="tr2:GetMaskOptionsResponse"/>
	</wsdl:message>
	<wsdl:message name="SetMaskRequest">
		<wsdl:part name="parameters" element="tr2:SetMask"/>
	</wsdl:message>
	<wsdl:message name="SetMaskResponse">
		<wsdl:part name="parameters" element="tr2:SetMaskResponse"/>
	</wsdl:message>
	<wsdl:message name="CreateMaskRequest">
		<wsdl:part name="parameters" element="tr2:CreateMask"/>
	</wsdl:message>
	<wsdl:message name="CreateMaskResponse">
		<wsdl:part name="parameters" element="tr2:CreateMaskResponse"/>
	</wsdl:message>
	<wsdl:message name="DeleteMaskRequest">
		<wsdl:part name="parameters" element="tr2:DeleteMask"/>
	</wsdl:message>
	<wsdl:message name="DeleteMaskResponse">
		<wsdl:part name="parameters" element="tr2:DeleteMaskResponse"/>
	</wsdl:message>
	<wsdl:portType name="Media2">
		<!--===============================-->
		<!--===============================-->
		<wsdl:operation name="GetServiceCapabilities">
			<wsdl:documentation>Returns the capabilities of the media service. The result is returned in a typed answer.</wsdl:documentation>
			<wsdl:input message="tr2:GetServiceCapabilitiesRequest"/>
			<wsdl:output message="tr2:GetServiceCapabilitiesResponse"/>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="CreateProfile">
			<wsdl:documentation>This operation creates a new media profile. 
				A created profile created via this method may be deleted via the DeleteProfile method.
				Optionally Configurations can be assinged to the profile on creation. For details regarding profile assignement
				check also the method AddConfiguration.
			</wsdl:documentation>
			<wsdl:input message="tr2:CreateProfileRequest"/>
			<wsdl:output message="tr2:CreateProfileResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetProfiles">
			<wsdl:documentation>Retrieve the profile with the specified token or all defined media profiles.
				<ul>
					<li>If no Type is provided the returned profiles shall contain no configuration information.</li>
					<li>If a single Type with value 'All' is provided the returned profiles shall include all associated configurations.</li>
					<li>Otherwise the requested list of configurations shall for each profile include the configurations present as Type.</li>
				</ul>
			</wsdl:documentation>
			<wsdl:input message="tr2:GetProfilesRequest"/>
			<wsdl:output message="tr2:GetProfilesResponse"/>
		</wsdl:operation>
		<wsdl:operation name="AddConfiguration">
			<wsdl:documentation>This operation adds one or more Configurations to an existing media profile. If a
configuration exists in the media profile, it will be replaced. A device shall
support adding a compatible Configuration to a Profile containing a VideoSourceConfiguration and shall
support streaming video data of such a profile.<br/>
				Note that OSD elements must be added via the CreateOSD command.
			</wsdl:documentation>
			<wsdl:input message="tr2:AddConfigurationRequest"/>
			<wsdl:output message="tr2:AddConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="RemoveConfiguration">
			<wsdl:documentation>This operation removes the listed configurations from an existing media profile. If the
				media profile does not contain one of the listed configurations that item shall be ignored.</wsdl:documentation>
			<wsdl:input message="tr2:RemoveConfigurationRequest"/>
			<wsdl:output message="tr2:RemoveConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="DeleteProfile">
			<wsdl:documentation>This operation deletes a profile. Deletion of a profile is only possible for non-fixed profiles</wsdl:documentation>
			<wsdl:input message="tr2:DeleteProfileRequest"/>
			<wsdl:output message="tr2:DeleteProfileResponse"/>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetVideoSourceConfigurations">
			<wsdl:documentation>By default this operation lists all existing video source configurations for a device. Provide a profile token to list only configurations that are compatible with the profile. If a configuration token is provided only a single configuration will be returned.</wsdl:documentation>
			<wsdl:input message="tr2:GetVideoSourceConfigurationsRequest"/>
			<wsdl:output message="tr2:GetVideoSourceConfigurationsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetVideoEncoderConfigurations">
			<wsdl:documentation>By default this operation lists all existing video encoder configurations for a device. Provide a profile token to list only configurations that are compatible with the profile. If a configuration token is provided only a single configuration will be returned.</wsdl:documentation>
			<wsdl:input message="tr2:GetVideoEncoderConfigurationsRequest"/>
			<wsdl:output message="tr2:GetVideoEncoderConfigurationsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetAudioSourceConfigurations">
			<wsdl:documentation>By default this operation lists all existing audio source configurations for a device. Provide a profile token to list only configurations that are compatible with the profile. If a configuration token is provided only a single configuration will be returned.</wsdl:documentation>
			<wsdl:input message="tr2:GetAudioSourceConfigurationsRequest"/>
			<wsdl:output message="tr2:GetAudioSourceConfigurationsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetAudioEncoderConfigurations">
			<wsdl:documentation>By default this operation lists all existing audio encoder configurations for a device. Provide a profile token to list only configurations that are compatible with the profile. If a configuration token is provided only a single configuration will be returned.</wsdl:documentation>
			<wsdl:input message="tr2:GetAudioEncoderConfigurationsRequest"/>
			<wsdl:output message="tr2:GetAudioEncoderConfigurationsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetAnalyticsConfigurations">
			<wsdl:documentation>By default this operation lists all existing video analytics configurations for a device. Provide a profile token to list only configurations that are compatible with the profile. If a configuration token is provided only a single configuration will be returned.</wsdl:documentation>
			<wsdl:input message="tr2:GetAnalyticsConfigurationsRequest"/>
			<wsdl:output message="tr2:GetAnalyticsConfigurationsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetMetadataConfigurations">
			<wsdl:documentation>By default this operation lists all existing metadata configurations for a device. Provide a profile token to list only configurations that are compatible with the profile. If a configuration token is provided only a single configuration will be returned.</wsdl:documentation>
			<wsdl:input message="tr2:GetMetadataConfigurationsRequest"/>
			<wsdl:output message="tr2:GetMetadataConfigurationsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetAudioOutputConfigurations">
			<wsdl:documentation>By default this operation lists all existing audio output configurations for a device. Provide a profile token to list only configurations that are compatible with the profile. If a configuration token is provided only a single configuration will be returned.</wsdl:documentation>
			<wsdl:input message="tr2:GetAudioOutputConfigurationsRequest"/>
			<wsdl:output message="tr2:GetAudioOutputConfigurationsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetAudioDecoderConfigurations">
			<wsdl:documentation>By default this operation lists all existing audio decoder configurations for a device. Provide a profile token to list only configurations that are compatible with the profile. If a configuration token is provided only a single configuration will be returned.</wsdl:documentation>
			<wsdl:input message="tr2:GetAudioDecoderConfigurationsRequest"/>
			<wsdl:output message="tr2:GetAudioDecoderConfigurationsResponse"/>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="SetVideoSourceConfiguration">
			<wsdl:documentation>This operation modifies a video source configuration. Running streams using this configuration may be immediately updated according to the new settings. The changes are not guaranteed to take effect unless the client requests a new stream URI and restarts any affected stream. NVC methods for changing a running stream are out of scope for this specification.</wsdl:documentation>
			<wsdl:input message="tr2:SetVideoSourceConfigurationRequest"/>
			<wsdl:output message="tr2:SetVideoSourceConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetVideoEncoderConfiguration">
			<wsdl:documentation>This operation modifies a video encoder configuration. Running streams using this configuration may be immediately updated according to the new settings. The changes are not guaranteed to take effect unless the client requests a new stream URI and restarts any affected stream. NVC methods for changing a running stream are out of scope for this specification. <br/>SessionTimeout is provided as a hint for keeping rtsp session by a device. If necessary the device may adapt parameter values for SessionTimeout elements without returning an error. For the time between keep alive calls the client shall adhere to the timeout value signaled via RTSP.</wsdl:documentation>
			<wsdl:input message="tr2:SetVideoEncoderConfigurationRequest"/>
			<wsdl:output message="tr2:SetVideoEncoderConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetAudioSourceConfiguration">
			<wsdl:documentation>This operation modifies an audio source configuration. Running streams using this configuration
				may be immediately updated according to the new settings. The changes are not guaranteed
				to take effect unless the client requests a new stream URI and restarts any affected stream
				NVC methods for changing a running stream are out of scope for this specification.</wsdl:documentation>
			<wsdl:input message="tr2:SetAudioSourceConfigurationRequest"/>
			<wsdl:output message="tr2:SetAudioSourceConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetAudioEncoderConfiguration">
			<wsdl:documentation>This operation modifies an audio encoder configuration. Running streams using this configuration may be immediately updated
				according to the new settings. The changes are not guaranteed to take effect unless the client
				requests a new stream URI and restarts any affected streams. NVC methods for changing a
				running stream are out of scope for this specification.</wsdl:documentation>
			<wsdl:input message="tr2:SetAudioEncoderConfigurationRequest"/>
			<wsdl:output message="tr2:SetAudioEncoderConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetMetadataConfiguration">
			<wsdl:documentation>This operation modifies a metadata configuration. Running streams using this configuration may be updated immediately
				according to the new settings. The changes are not guaranteed to take effect unless the client
				requests a new stream URI and restarts any affected streams. NVC methods for changing a
				running stream are out of scope for this specification.</wsdl:documentation>
			<wsdl:input message="tr2:SetMetadataConfigurationRequest"/>
			<wsdl:output message="tr2:SetMetadataConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetAudioOutputConfiguration">
			<wsdl:documentation>This operation modifies an audio output configuration.</wsdl:documentation>
			<wsdl:input message="tr2:SetAudioOutputConfigurationRequest"/>
			<wsdl:output message="tr2:SetAudioOutputConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetAudioDecoderConfiguration">
			<wsdl:documentation>This operation modifies an audio decoder configuration.</wsdl:documentation>
			<wsdl:input message="tr2:SetAudioDecoderConfigurationRequest"/>
			<wsdl:output message="tr2:SetAudioDecoderConfigurationResponse"/>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetVideoSourceConfigurationOptions">
			<wsdl:documentation>This operation returns the available options  (supported values and ranges for video source configuration parameters) when the video source parameters are
				reconfigured If a video source configuration is specified, the options shall concern that
				particular configuration. If a media profile is specified, the options shall be compatible with
				that media profile.</wsdl:documentation>
			<wsdl:input message="tr2:GetVideoSourceConfigurationOptionsRequest"/>
			<wsdl:output message="tr2:GetVideoSourceConfigurationOptionsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetVideoEncoderConfigurationOptions">
			<wsdl:documentation>This operation returns the available options (supported values and ranges for video encoder 
				configuration parameters) when the video encoder parameters are reconfigured. <br/>
				This response contains the available video encoder configuration options. If a video encoder configuration is specified, 
				the options shall concern that particular configuration. If a media profile is specified, the options shall be 
				compatible with that media profile. If no tokens are specified, the options shall be considered generic for the device.
			</wsdl:documentation>
			<wsdl:input message="tr2:GetVideoEncoderConfigurationOptionsRequest"/>
			<wsdl:output message="tr2:GetVideoEncoderConfigurationOptionsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetAudioSourceConfigurationOptions">
			<wsdl:documentation>This operation returns the available options (supported values and ranges for audio source configuration parameters) when the audio source parameters are
				reconfigured. If an audio source configuration is specified, the options shall concern that
				particular configuration. If a media profile is specified, the options shall be compatible with
				that media profile.</wsdl:documentation>
			<wsdl:input message="tr2:GetAudioSourceConfigurationOptionsRequest"/>
			<wsdl:output message="tr2:GetAudioSourceConfigurationOptionsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetAudioEncoderConfigurationOptions">
			<wsdl:documentation>This operation returns the available options  (supported values and ranges for audio encoder configuration parameters) when the audio encoder parameters are
				reconfigured.</wsdl:documentation>
			<wsdl:input message="tr2:GetAudioEncoderConfigurationOptionsRequest"/>
			<wsdl:output message="tr2:GetAudioEncoderConfigurationOptionsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetMetadataConfigurationOptions">
			<wsdl:documentation>This operation returns the available options (supported values and ranges for metadata configuration parameters) for changing the metadata configuration.</wsdl:documentation>
			<wsdl:input message="tr2:GetMetadataConfigurationOptionsRequest"/>
			<wsdl:output message="tr2:GetMetadataConfigurationOptionsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetAudioOutputConfigurationOptions">
			<wsdl:documentation>This operation returns the available options (supported values and ranges for audio output configuration parameters) for configuring an audio output.</wsdl:documentation>
			<wsdl:input message="tr2:GetAudioOutputConfigurationOptionsRequest"/>
			<wsdl:output message="tr2:GetAudioOutputConfigurationOptionsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetAudioDecoderConfigurationOptions">
			<wsdl:documentation>This command list the audio decoding capabilities for a given profile and configuration of a
				device.</wsdl:documentation>
			<wsdl:input message="tr2:GetAudioDecoderConfigurationOptionsRequest"/>
			<wsdl:output message="tr2:GetAudioDecoderConfigurationOptionsResponse"/>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetVideoEncoderInstances">
			<wsdl:documentation>The GetVideoEncoderInstances command can be used to request the
				minimum number of guaranteed video encoder instances (applications) per Video Source
				Configuration.</wsdl:documentation>
			<wsdl:input message="tr2:GetVideoEncoderInstancesRequest"/>
			<wsdl:output message="tr2:GetVideoEncoderInstancesResponse"/>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetStreamUri">
			<wsdl:documentation>This operation requests a URI that can be used to initiate a live media stream using RTSP as
				the control protocol. The returned URI shall remain valid indefinitely even if the profile is changed. <br/>
				Defined stream types are
				 <ul>
				 	<li>RtspUnicast		RTSP streaming RTP as UDP Unicast.</li>
				 	<li>RtspMulticast	RTSP streaming RTP as UDP Multicast.</li>
				 	<li>RTSP			RTSP streaming RTP over TCP.</li>
				 	<li>RtspOverHttp	Tunneling both the RTSP control channel and the RTP stream over HTTP or HTTPS.</li>
				 </ul>
				If a multicast stream is requested at least one of VideoEncoder2Configuration, AudioEncoder2Configuration and MetadataConfiguration shall have a valid multicast setting.<br/>
				For full compatibility with other ONVIF services a device should not generate Uris longer than
				128 octets.</wsdl:documentation>
			<wsdl:input message="tr2:GetStreamUriRequest"/>
			<wsdl:output message="tr2:GetStreamUriResponse"/>
		</wsdl:operation>
		<wsdl:operation name="StartMulticastStreaming">
			<wsdl:documentation>This command starts multicast streaming using a specified media profile of a device.
				Streaming continues until StopMulticastStreaming is called for the same Profile. The
				streaming shall continue after a reboot of the device until a StopMulticastStreaming request is
				received. The multicast address, port and TTL are configured in the
				VideoEncoderConfiguration, AudioEncoderConfiguration and MetadataConfiguration
				respectively.</wsdl:documentation>
			<wsdl:input message="tr2:StartMulticastStreamingRequest"/>
			<wsdl:output message="tr2:StartMulticastStreamingResponse"/>
		</wsdl:operation>
		<wsdl:operation name="StopMulticastStreaming">
			<wsdl:documentation>This command stops multicast streaming using a specified media profile of a device</wsdl:documentation>
			<wsdl:input message="tr2:StopMulticastStreamingRequest"/>
			<wsdl:output message="tr2:StopMulticastStreamingResponse"/>
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
			<wsdl:input message="tr2:SetSynchronizationPointRequest"/>
			<wsdl:output message="tr2:SetSynchronizationPointResponse"/>
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
			<wsdl:input message="tr2:GetSnapshotUriRequest"/>
			<wsdl:output message="tr2:GetSnapshotUriResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetVideoSourceModes">
			<wsdl:documentation>A device returns the information for current video source mode and settable video source modes of specified video source. A device that indicates a capability of  VideoSourceModes shall support this command.</wsdl:documentation>
			<wsdl:input message="tr2:GetVideoSourceModesRequest"/>
			<wsdl:output message="tr2:GetVideoSourceModesResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetVideoSourceMode">
			<wsdl:documentation>SetVideoSourceMode changes the media profile structure relating to video source for the specified video source mode. A device that indicates a capability of VideoSourceModes shall support this command. The behavior after changing the mode is not defined in this specification.</wsdl:documentation>
			<wsdl:input message="tr2:SetVideoSourceModeRequest"/>
			<wsdl:output message="tr2:SetVideoSourceModeResponse"/>
		</wsdl:operation>
		<!--==============OSD Operation Begin=================-->
		<wsdl:operation name="GetOSDs">
			<wsdl:documentation>This operation lists existing OSD configurations for the device.
				<ul>
					<li>If an OSD token is provided the device shall respond with the requested configuration or provide an error if it does not exist.</li>
					<li>In case only a video source configuration token is provided the device shall respond with all configurations that exist for the video source configuration.</li>
					<li>If no tokens are provided the device shall respond with all available OSD configurations.</li>
				</ul>
			</wsdl:documentation>
			<wsdl:input message="tr2:GetOSDsRequest"/>
			<wsdl:output message="tr2:GetOSDsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetOSDOptions">
			<wsdl:documentation>Get the OSD Options.</wsdl:documentation>
			<wsdl:input message="tr2:GetOSDOptionsRequest"/>
			<wsdl:output message="tr2:GetOSDOptionsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetOSD">
			<wsdl:documentation>Set the OSD</wsdl:documentation>
			<wsdl:input message="tr2:SetOSDRequest"/>
			<wsdl:output message="tr2:SetOSDResponse"/>
		</wsdl:operation>
		<wsdl:operation name="CreateOSD">
			<wsdl:documentation>Create the OSD.</wsdl:documentation>
			<wsdl:input message="tr2:CreateOSDRequest"/>
			<wsdl:output message="tr2:CreateOSDResponse"/>
		</wsdl:operation>
		<wsdl:operation name="DeleteOSD">
			<wsdl:documentation>Delete the OSD.</wsdl:documentation>
			<wsdl:input message="tr2:DeleteOSDRequest"/>
			<wsdl:output message="tr2:DeleteOSDResponse"/>
		</wsdl:operation>
		<!--==============OSD Operation End=================-->
		<!--==============Mask Operation Begin=================-->
		<wsdl:operation name="GetMasks">
			<wsdl:documentation>This operation lists existing Mask configurations for the device.
				<ul>
					<li>If an Mask token is provided the device shall respond with the requested configuration or provide an error if it does not exist.</li>
					<li>In case only a video source configuration token is provided the device shall respond with all configurations that exist for the video source configuration.</li>
					<li>If no tokens are provided the device shall respond with all available Mask configurations.</li>
				</ul>
			</wsdl:documentation>
			<wsdl:input message="tr2:GetMasksRequest"/>
			<wsdl:output message="tr2:GetMasksResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetMaskOptions">
			<wsdl:documentation>Get the Mask Options.</wsdl:documentation>
			<wsdl:input message="tr2:GetMaskOptionsRequest"/>
			<wsdl:output message="tr2:GetMaskOptionsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetMask">
			<wsdl:documentation>Set the Mask</wsdl:documentation>
			<wsdl:input message="tr2:SetMaskRequest"/>
			<wsdl:output message="tr2:SetMaskResponse"/>
		</wsdl:operation>
		<wsdl:operation name="CreateMask">
			<wsdl:documentation>Create the Mask.</wsdl:documentation>
			<wsdl:input message="tr2:CreateMaskRequest"/>
			<wsdl:output message="tr2:CreateMaskResponse"/>
		</wsdl:operation>
		<wsdl:operation name="DeleteMask">
			<wsdl:documentation>Delete the Mask.</wsdl:documentation>
			<wsdl:input message="tr2:DeleteMaskRequest"/>
			<wsdl:output message="tr2:DeleteMaskResponse"/>
		</wsdl:operation>
		<!--==============Mask Operation End=================-->
	</wsdl:portType>
	<wsdl:binding name="Media2Binding" type="tr2:Media2">
		<soap:binding style="document" transport="http://schemas.xmlsoap.org/soap/http"/>
		<!--===============================-->
		<!--===============================-->
		<wsdl:operation name="GetServiceCapabilities">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/GetServiceCapabilities"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="CreateProfile">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/CreateProfile"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetProfiles">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/GetProfiles"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="AddConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/AddConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="RemoveConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/RemoveConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="DeleteProfile">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/DeleteProfile"/>
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
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/GetVideoSourceConfigurations"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetVideoEncoderConfigurations">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/GetVideoEncoderConfigurations"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetAudioSourceConfigurations">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/GetAudioSourceConfigurations/"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetAudioEncoderConfigurations">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/GetAudioEncoderConfigurations"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetAnalyticsConfigurations">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/GetAnalyticsConfigurations"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetMetadataConfigurations">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/GetMetadataConfigurations"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetAudioOutputConfigurations">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/GetAudioOutputConfigurations"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetAudioDecoderConfigurations">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/GetAudioDecoderConfigurations"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="SetVideoSourceConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/SetVideoSourceConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetVideoEncoderConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/SetVideoEncoderConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetAudioSourceConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/SetAudioSourceConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetAudioEncoderConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/SetAudioEncoderConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetMetadataConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/SetMetadataConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetAudioOutputConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/SetAudioOutputConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetAudioDecoderConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/SetAudioDecoderConfiguration"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetVideoSourceConfigurationOptions">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/GetVideoSourceConfigurationOptions/"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetVideoEncoderConfigurationOptions">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/GetVideoEncoderConfigurationOptions"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetAudioSourceConfigurationOptions">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/GetAudioSourceConfigurationOptions"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetAudioEncoderConfigurationOptions">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/GetAudioEncoderConfigurationOptions"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetMetadataConfigurationOptions">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/GetMetadataConfigurationOptions"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetAudioOutputConfigurationOptions">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/GetAudioOutputConfigurationOptions"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetAudioDecoderConfigurationOptions">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/GetAudioDecoderConfigurationOptions"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetVideoEncoderInstances">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/GetVideoEncoderInstances"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetStreamUri">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/GetStreamUri"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="StartMulticastStreaming">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/StartMulticastStreaming"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="StopMulticastStreaming">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/StopMulticastStreaming"/>
			<wsdl:input>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body parts="parameters" use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="SetSynchronizationPoint">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/SetSynchronizationPoint"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetSnapshotUri">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/GetSnapshotUri"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetVideoSourceModes">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/GetVideoSourceModes"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="SetVideoSourceMode">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/SetVideoSourceMode"/>
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
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/GetOSDs"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetOSDOptions">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/GetOSDOptions"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetOSD">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/SetOSD"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="CreateOSD">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/CreateOSD"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="DeleteOSD">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/DeleteOSD"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--==============OSD Operation End=================-->
		<!--==============Mask Operation Begin=================-->
		<wsdl:operation name="GetMasks">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/GetMasks"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetMaskOptions">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/GetMaskOptions"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetMask">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/SetMask"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="CreateMask">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/CreateMask"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="DeleteMask">
			<soap:operation soapAction="http://www.onvif.org/ver20/media/wsdl/DeleteMask"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--==============Mask Operation End=================-->
	</wsdl:binding>
</wsdl:definitions>
